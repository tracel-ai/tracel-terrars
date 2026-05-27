use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct BeyondcorpSecurityGatewayData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    security_gateway_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hubs: Option<Vec<BeyondcorpSecurityGatewayHubsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging: Option<Vec<BeyondcorpSecurityGatewayLoggingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proxy_protocol_config: Option<Vec<BeyondcorpSecurityGatewayProxyProtocolConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_discovery: Option<Vec<BeyondcorpSecurityGatewayServiceDiscoveryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<BeyondcorpSecurityGatewayTimeoutsEl>,
    dynamic: BeyondcorpSecurityGatewayDynamic,
}
struct BeyondcorpSecurityGateway_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BeyondcorpSecurityGatewayData>,
}
#[derive(Clone)]
pub struct BeyondcorpSecurityGateway(Rc<BeyondcorpSecurityGateway_>);
impl BeyondcorpSecurityGateway {
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
    #[doc = "Set the field `display_name`.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn set_location(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().location = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `hubs`.\n"]
    pub fn set_hubs(self, v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayHubsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hubs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hubs = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayLoggingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `proxy_protocol_config`.\n"]
    pub fn set_proxy_protocol_config(
        self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayProxyProtocolConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_protocol_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_protocol_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `service_discovery`.\n"]
    pub fn set_service_discovery(
        self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayServiceDiscoveryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_discovery = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_discovery = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<BeyondcorpSecurityGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delegating_service_account` after provisioning.\nService account used for operations that involve resources in consumer projects."]
    pub fn delegating_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegating_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ips` after provisioning.\nOutput only. IP addresses that will be used for establishing\nconnection to the endpoints."]
    pub fn external_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The operational state of the SecurityGateway.\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nUPDATING\nDELETING\nRUNNING\nDOWN\nERROR"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<BeyondcorpSecurityGatewayLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_protocol_config` after provisioning.\n"]
    pub fn proxy_protocol_config(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayProxyProtocolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_protocol_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> ListRef<BeyondcorpSecurityGatewayServiceDiscoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_discovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGatewayTimeoutsElRef {
        BeyondcorpSecurityGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for BeyondcorpSecurityGateway {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for BeyondcorpSecurityGateway {}
impl ToListMappable for BeyondcorpSecurityGateway {
    type O = ListRef<BeyondcorpSecurityGatewayRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for BeyondcorpSecurityGateway_ {
    fn extract_resource_type(&self) -> String {
        "google_beyondcorp_security_gateway".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildBeyondcorpSecurityGateway {
    pub tf_id: String,
    #[doc = "Optional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub security_gateway_id: PrimField<String>,
}
impl BuildBeyondcorpSecurityGateway {
    pub fn build(self, stack: &mut Stack) -> BeyondcorpSecurityGateway {
        let out = BeyondcorpSecurityGateway(Rc::new(BeyondcorpSecurityGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BeyondcorpSecurityGatewayData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: core::default::Default::default(),
                project: core::default::Default::default(),
                security_gateway_id: self.security_gateway_id,
                hubs: core::default::Default::default(),
                logging: core::default::Default::default(),
                proxy_protocol_config: core::default::Default::default(),
                service_discovery: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct BeyondcorpSecurityGatewayRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl BeyondcorpSecurityGatewayRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. Timestamp when the resource was created."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `delegating_service_account` after provisioning.\nService account used for operations that involve resources in consumer projects."]
    pub fn delegating_service_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.delegating_service_account", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nOptional. An arbitrary user-provided name for the SecurityGateway.\nCannot exceed 64 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `external_ips` after provisioning.\nOutput only. IP addresses that will be used for establishing\nconnection to the endpoints."]
    pub fn external_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.external_ips", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nResource ID segment making up resource 'name'. It identifies the resource within its parent collection as described in https://google.aip.dev/122. Must be omitted or set to 'global'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
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
    #[doc = "Get a reference to the value of field `security_gateway_id` after provisioning.\nOptional. User-settable SecurityGateway resource ID.\n* Must start with a letter.\n* Must contain between 4-63 characters from '/a-z-/'.\n* Must end with a number or letter."]
    pub fn security_gateway_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.security_gateway_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\nOutput only. The operational state of the SecurityGateway.\nPossible values:\nSTATE_UNSPECIFIED\nCREATING\nUPDATING\nDELETING\nRUNNING\nDOWN\nERROR"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. Timestamp when the resource was last modified."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> ListRef<BeyondcorpSecurityGatewayLoggingElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logging", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `proxy_protocol_config` after provisioning.\n"]
    pub fn proxy_protocol_config(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayProxyProtocolConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.proxy_protocol_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_discovery` after provisioning.\n"]
    pub fn service_discovery(&self) -> ListRef<BeyondcorpSecurityGatewayServiceDiscoveryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.service_discovery", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BeyondcorpSecurityGatewayTimeoutsElRef {
        BeyondcorpSecurityGatewayTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayHubsElInternetGatewayEl {}
impl BeyondcorpSecurityGatewayHubsElInternetGatewayEl {}
impl ToListMappable for BeyondcorpSecurityGatewayHubsElInternetGatewayEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayHubsElInternetGatewayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayHubsElInternetGatewayEl {}
impl BuildBeyondcorpSecurityGatewayHubsElInternetGatewayEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayHubsElInternetGatewayEl {
        BeyondcorpSecurityGatewayHubsElInternetGatewayEl {}
    }
}
pub struct BeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
        BeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayHubsElInternetGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `assigned_ips` after provisioning.\nOutput only. List of IP addresses assigned to the Cloud NAT."]
    pub fn assigned_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.assigned_ips", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayHubsElDynamic {
    internet_gateway: Option<DynamicBlock<BeyondcorpSecurityGatewayHubsElInternetGatewayEl>>,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayHubsEl {
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    internet_gateway: Option<Vec<BeyondcorpSecurityGatewayHubsElInternetGatewayEl>>,
    dynamic: BeyondcorpSecurityGatewayHubsElDynamic,
}
impl BeyondcorpSecurityGatewayHubsEl {
    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayHubsElInternetGatewayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.internet_gateway = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.internet_gateway = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayHubsEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayHubsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayHubsEl {
    #[doc = ""]
    pub region: PrimField<String>,
}
impl BuildBeyondcorpSecurityGatewayHubsEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayHubsEl {
        BeyondcorpSecurityGatewayHubsEl {
            region: self.region,
            internet_gateway: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayHubsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayHubsElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpSecurityGatewayHubsElRef {
        BeyondcorpSecurityGatewayHubsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayHubsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> ListRef<BeyondcorpSecurityGatewayHubsElInternetGatewayElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.internet_gateway", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayLoggingEl {}
impl BeyondcorpSecurityGatewayLoggingEl {}
impl ToListMappable for BeyondcorpSecurityGatewayLoggingEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayLoggingEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayLoggingEl {}
impl BuildBeyondcorpSecurityGatewayLoggingEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayLoggingEl {
        BeyondcorpSecurityGatewayLoggingEl {}
    }
}
pub struct BeyondcorpSecurityGatewayLoggingElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayLoggingElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpSecurityGatewayLoggingElRef {
        BeyondcorpSecurityGatewayLoggingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayLoggingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
    #[doc = "Set the field `output_type`.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl
{
    type O = BlockAssignable<
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {}
impl BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
    pub fn build(
        self,
    ) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl {
            output_type: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated device info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
    #[doc = "Set the field `output_type`.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl
{
    type O = BlockAssignable<
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {}
impl BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
    pub fn build(
        self,
    ) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl {
            output_type: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated group info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
    #[doc = "Set the field `output_type`.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
}
impl ToListMappable
    for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl
{
    type O = BlockAssignable<
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {}
impl BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
    pub fn build(
        self,
    ) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl {
            output_type: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nThe output type of the delegated user info. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDynamic {
    device_info: Option<
        DynamicBlock<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl>,
    >,
    group_info: Option<
        DynamicBlock<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl>,
    >,
    user_info: Option<
        DynamicBlock<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl>,
    >,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    output_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_info:
        Option<Vec<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group_info:
        Option<Vec<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_info:
        Option<Vec<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl>>,
    dynamic: BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDynamic,
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    #[doc = "Set the field `output_type`.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn set_output_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.output_type = Some(v.into());
        self
    }
    #[doc = "Set the field `device_info`.\n"]
    pub fn set_device_info(
        mut self,
        v: impl Into<
            BlockAssignable<
                BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoEl,
            >,
        >,
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
        v: impl Into<
            BlockAssignable<
                BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoEl,
            >,
        >,
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
        v: impl Into<
            BlockAssignable<
                BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoEl,
            >,
        >,
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
impl ToListMappable for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {}
impl BuildBeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl {
            output_type: core::default::Default::default(),
            device_info: core::default::Default::default(),
            group_info: core::default::Default::default(),
            user_info: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
        BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `output_type` after provisioning.\nDefault output type for all enabled headers. Possible values: [\"PROTOBUF\", \"JSON\", \"NONE\"]"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.base))
    }
    #[doc = "Get a reference to the value of field `device_info` after provisioning.\n"]
    pub fn device_info(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElDeviceInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.device_info", self.base))
    }
    #[doc = "Get a reference to the value of field `group_info` after provisioning.\n"]
    pub fn group_info(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElGroupInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.group_info", self.base))
    }
    #[doc = "Get a reference to the value of field `user_info` after provisioning.\n"]
    pub fn user_info(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElUserInfoElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.user_info", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayProxyProtocolConfigElDynamic {
    contextual_headers:
        Option<DynamicBlock<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl>>,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_client_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gateway_identity: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata_headers: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contextual_headers:
        Option<Vec<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl>>,
    dynamic: BeyondcorpSecurityGatewayProxyProtocolConfigElDynamic,
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigEl {
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
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersEl>>,
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
impl ToListMappable for BeyondcorpSecurityGatewayProxyProtocolConfigEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayProxyProtocolConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayProxyProtocolConfigEl {}
impl BuildBeyondcorpSecurityGatewayProxyProtocolConfigEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayProxyProtocolConfigEl {
        BeyondcorpSecurityGatewayProxyProtocolConfigEl {
            allowed_client_headers: core::default::Default::default(),
            client_ip: core::default::Default::default(),
            gateway_identity: core::default::Default::default(),
            metadata_headers: core::default::Default::default(),
            contextual_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayProxyProtocolConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayProxyProtocolConfigElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpSecurityGatewayProxyProtocolConfigElRef {
        BeyondcorpSecurityGatewayProxyProtocolConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayProxyProtocolConfigElRef {
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
    ) -> ListRef<BeyondcorpSecurityGatewayProxyProtocolConfigElContextualHeadersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.contextual_headers", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
    #[doc = "Set the field `path`.\nContains uri path fragment where HTTP request is sent."]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
    type O =
        BlockAssignable<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {}
impl BuildBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
    pub fn build(
        self,
    ) -> BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
        BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl {
            path: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
        BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\nContains uri path fragment where HTTP request is sent."]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElDynamic {
    resource_override: Option<
        DynamicBlock<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl>,
    >,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_override:
        Option<Vec<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl>>,
    dynamic: BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElDynamic,
}
impl BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    #[doc = "Set the field `resource_override`.\n"]
    pub fn set_resource_override(
        mut self,
        v: impl Into<
            BlockAssignable<
                BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.resource_override = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_override = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {}
impl BuildBeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
        BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl {
            resource_override: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
        BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_override` after provisioning.\n"]
    pub fn resource_override(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElResourceOverrideElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_override", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct BeyondcorpSecurityGatewayServiceDiscoveryElDynamic {
    api_gateway: Option<DynamicBlock<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl>>,
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayServiceDiscoveryEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    api_gateway: Option<Vec<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl>>,
    dynamic: BeyondcorpSecurityGatewayServiceDiscoveryElDynamic,
}
impl BeyondcorpSecurityGatewayServiceDiscoveryEl {
    #[doc = "Set the field `api_gateway`.\n"]
    pub fn set_api_gateway(
        mut self,
        v: impl Into<BlockAssignable<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.api_gateway = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.api_gateway = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for BeyondcorpSecurityGatewayServiceDiscoveryEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayServiceDiscoveryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayServiceDiscoveryEl {}
impl BuildBeyondcorpSecurityGatewayServiceDiscoveryEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayServiceDiscoveryEl {
        BeyondcorpSecurityGatewayServiceDiscoveryEl {
            api_gateway: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayServiceDiscoveryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayServiceDiscoveryElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpSecurityGatewayServiceDiscoveryElRef {
        BeyondcorpSecurityGatewayServiceDiscoveryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayServiceDiscoveryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `api_gateway` after provisioning.\n"]
    pub fn api_gateway(
        &self,
    ) -> ListRef<BeyondcorpSecurityGatewayServiceDiscoveryElApiGatewayElRef> {
        ListRef::new(self.shared().clone(), format!("{}.api_gateway", self.base))
    }
}
#[derive(Serialize)]
pub struct BeyondcorpSecurityGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl BeyondcorpSecurityGatewayTimeoutsEl {
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
impl ToListMappable for BeyondcorpSecurityGatewayTimeoutsEl {
    type O = BlockAssignable<BeyondcorpSecurityGatewayTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildBeyondcorpSecurityGatewayTimeoutsEl {}
impl BuildBeyondcorpSecurityGatewayTimeoutsEl {
    pub fn build(self) -> BeyondcorpSecurityGatewayTimeoutsEl {
        BeyondcorpSecurityGatewayTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct BeyondcorpSecurityGatewayTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for BeyondcorpSecurityGatewayTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> BeyondcorpSecurityGatewayTimeoutsElRef {
        BeyondcorpSecurityGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl BeyondcorpSecurityGatewayTimeoutsElRef {
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
struct BeyondcorpSecurityGatewayDynamic {
    hubs: Option<DynamicBlock<BeyondcorpSecurityGatewayHubsEl>>,
    logging: Option<DynamicBlock<BeyondcorpSecurityGatewayLoggingEl>>,
    proxy_protocol_config: Option<DynamicBlock<BeyondcorpSecurityGatewayProxyProtocolConfigEl>>,
    service_discovery: Option<DynamicBlock<BeyondcorpSecurityGatewayServiceDiscoveryEl>>,
}
