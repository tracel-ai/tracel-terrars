use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MemorystoreInstanceDesiredUserCreatedEndpointsData {
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
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    region: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    desired_user_created_endpoints:
        Option<Vec<MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl>,
    dynamic: MemorystoreInstanceDesiredUserCreatedEndpointsDynamic,
}
struct MemorystoreInstanceDesiredUserCreatedEndpoints_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MemorystoreInstanceDesiredUserCreatedEndpointsData>,
}
#[derive(Clone)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpoints(
    Rc<MemorystoreInstanceDesiredUserCreatedEndpoints_>,
);
impl MemorystoreInstanceDesiredUserCreatedEndpoints {
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
    #[doc = "Set the field `desired_user_created_endpoints`.\n"]
    pub fn set_desired_user_created_endpoints(
        self,
        v: impl Into<
            BlockAssignable<
                MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().desired_user_created_endpoints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .desired_user_created_endpoints = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl>,
    ) -> Self {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Memorystore instance these endpoints should be added to."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Memorystore instance these endpoints should be added to."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_user_created_endpoints` after provisioning.\n"]
    pub fn desired_user_created_endpoints(
        &self,
    ) -> ListRef<MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_user_created_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef {
        MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for MemorystoreInstanceDesiredUserCreatedEndpoints {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MemorystoreInstanceDesiredUserCreatedEndpoints {}
impl ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpoints {
    type O = ListRef<MemorystoreInstanceDesiredUserCreatedEndpointsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MemorystoreInstanceDesiredUserCreatedEndpoints_ {
    fn extract_resource_type(&self) -> String {
        "google_memorystore_instance_desired_user_created_endpoints".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpoints {
    pub tf_id: String,
    #[doc = "The name of the Memorystore instance these endpoints should be added to."]
    pub name: PrimField<String>,
    #[doc = "The name of the region of the Memorystore instance these endpoints should be added to."]
    pub region: PrimField<String>,
}
impl BuildMemorystoreInstanceDesiredUserCreatedEndpoints {
    pub fn build(self, stack: &mut Stack) -> MemorystoreInstanceDesiredUserCreatedEndpoints {
        let out = MemorystoreInstanceDesiredUserCreatedEndpoints(Rc::new(
            MemorystoreInstanceDesiredUserCreatedEndpoints_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(MemorystoreInstanceDesiredUserCreatedEndpointsData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    deletion_policy: core::default::Default::default(),
                    id: core::default::Default::default(),
                    name: self.name,
                    project: core::default::Default::default(),
                    region: self.region,
                    desired_user_created_endpoints: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceDesiredUserCreatedEndpointsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MemorystoreInstanceDesiredUserCreatedEndpointsRef {
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
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the Memorystore instance these endpoints should be added to."]
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
    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe name of the region of the Memorystore instance these endpoints should be added to."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `desired_user_created_endpoints` after provisioning.\n"]
    pub fn desired_user_created_endpoints(
        &self,
    ) -> ListRef<MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.desired_user_created_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef {
        MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl
{
    forwarding_rule: PrimField<String>,
    ip_address: PrimField<String>,
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_id: Option<PrimField<String>>,
    psc_connection_id: PrimField<String>,
    service_attachment: PrimField<String>,
}
impl MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl { # [doc = "Set the field `project_id`.\nThe consumer project_id where the forwarding rule is created from."] pub fn set_project_id (mut self , v : impl Into < PrimField < String > >) -> Self { self . project_id = Some (v . into ()) ; self } }
impl ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl { type O = BlockAssignable < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl
{
    #[doc = "The URI of the consumer side forwarding rule.\nFormat:\nprojects/{project}/regions/{region}/forwardingRules/{forwarding_rule}"]
    pub forwarding_rule: PrimField<String>,
    #[doc = "The IP allocated on the consumer network for the PSC forwarding rule."]
    pub ip_address: PrimField<String>,
    #[doc = "The consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."]
    pub network: PrimField<String>,
    #[doc = "The PSC connection id of the forwarding rule connected to the\nservice attachment."]
    pub psc_connection_id: PrimField<String>,
    #[doc = "The service attachment which is the target of the PSC connection, in the form of projects/{project-id}/regions/{region}/serviceAttachments/{service-attachment-id}."]
    pub service_attachment: PrimField<String>,
}
impl BuildMemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl { pub fn build (self) -> MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl { MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl { forwarding_rule : self . forwarding_rule , ip_address : self . ip_address , network : self . network , project_id : core :: default :: Default :: default () , psc_connection_id : self . psc_connection_id , service_attachment : self . service_attachment , } } }
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionElRef { fn new (shared : StackShared , base : String) -> MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionElRef { MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionElRef { shared : shared , base : base . to_string () , } } }
impl MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `connection_type` after provisioning.\nOutput Only. Type of a PSC Connection. \n Possible values:\n CONNECTION_TYPE_DISCOVERY \n CONNECTION_TYPE_PRIMARY \n CONNECTION_TYPE_READER"] pub fn connection_type (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.connection_type" , self . base)) } # [doc = "Get a reference to the value of field `forwarding_rule` after provisioning.\nThe URI of the consumer side forwarding rule.\nFormat:\nprojects/{project}/regions/{region}/forwardingRules/{forwarding_rule}"] pub fn forwarding_rule (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.forwarding_rule" , self . base)) } # [doc = "Get a reference to the value of field `ip_address` after provisioning.\nThe IP allocated on the consumer network for the PSC forwarding rule."] pub fn ip_address (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.ip_address" , self . base)) } # [doc = "Get a reference to the value of field `network` after provisioning.\nThe consumer network where the IP address resides, in the form of\nprojects/{project_id}/global/networks/{network_id}."] pub fn network (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.network" , self . base)) } # [doc = "Get a reference to the value of field `project_id` after provisioning.\nThe consumer project_id where the forwarding rule is created from."] pub fn project_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.project_id" , self . base)) } # [doc = "Get a reference to the value of field `psc_connection_id` after provisioning.\nThe PSC connection id of the forwarding rule connected to the\nservice attachment."] pub fn psc_connection_id (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.psc_connection_id" , self . base)) } # [doc = "Get a reference to the value of field `psc_connection_status` after provisioning.\nOutput Only. The status of the PSC connection: whether a connection exists and ACTIVE or it no longer exists. \n Possible values:\n ACTIVE \n NOT_FOUND"] pub fn psc_connection_status (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.psc_connection_status" , self . base)) } # [doc = "Get a reference to the value of field `service_attachment` after provisioning.\nThe service attachment which is the target of the PSC connection, in the form of projects/{project-id}/regions/{region}/serviceAttachments/{service-attachment-id}."] pub fn service_attachment (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.service_attachment" , self . base)) } }
#[derive(Serialize, Default)]
struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElDynamic { psc_connection : Option < DynamicBlock < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl >> , }
#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl { # [serde (skip_serializing_if = "Option::is_none")] psc_connection : Option < Vec < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl > > , dynamic : MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElDynamic , }
impl MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl {
    #[doc = "Set the field `psc_connection`.\n"]
    pub fn set_psc_connection(
        mut self,
        v : impl Into < BlockAssignable < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.psc_connection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.psc_connection = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl
{
    type O = BlockAssignable<
        MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl
{}
impl BuildMemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl {
    pub fn build(
        self,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl
    {
        MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl {
            psc_connection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElRef
    {
        MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElRef { shared : shared , base : base . to_string () , }
    }
}
impl MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `psc_connection` after provisioning.\n"]    pub fn psc_connection (& self) -> ListRef < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElPscConnectionElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.psc_connection", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElDynamic { connections : Option < DynamicBlock < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl >> , }
#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl { # [serde (skip_serializing_if = "Option::is_none")] connections : Option < Vec < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl > > , dynamic : MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElDynamic , }
impl MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl {
    #[doc = "Set the field `connections`.\n"]
    pub fn set_connections(
        mut self,
        v : impl Into < BlockAssignable < MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.connections = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.connections = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl
{
    type O = BlockAssignable<
        MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl {}
impl BuildMemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl {
    pub fn build(
        self,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl {
        MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl {
            connections: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElRef {
        MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `connections` after provisioning.\n"]
    pub fn connections(
        &self,
    ) -> ListRef<
        MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsElConnectionsElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.connections", self.base))
    }
}
#[derive(Serialize)]
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl {
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
impl ToListMappable for MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl {
    type O = BlockAssignable<MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl {}
impl BuildMemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl {
    pub fn build(self) -> MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl {
        MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef {
        MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MemorystoreInstanceDesiredUserCreatedEndpointsTimeoutsElRef {
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
struct MemorystoreInstanceDesiredUserCreatedEndpointsDynamic {
    desired_user_created_endpoints: Option<
        DynamicBlock<MemorystoreInstanceDesiredUserCreatedEndpointsDesiredUserCreatedEndpointsEl>,
    >,
}
