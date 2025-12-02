use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct LightsailContainerServiceDeploymentVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container: Option<Vec<LightsailContainerServiceDeploymentVersionContainerEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_endpoint: Option<Vec<LightsailContainerServiceDeploymentVersionPublicEndpointEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<LightsailContainerServiceDeploymentVersionTimeoutsEl>,
    dynamic: LightsailContainerServiceDeploymentVersionDynamic,
}
struct LightsailContainerServiceDeploymentVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<LightsailContainerServiceDeploymentVersionData>,
}
#[derive(Clone)]
pub struct LightsailContainerServiceDeploymentVersion(
    Rc<LightsailContainerServiceDeploymentVersion_>,
);
impl LightsailContainerServiceDeploymentVersion {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        self,
        v: impl Into<BlockAssignable<LightsailContainerServiceDeploymentVersionContainerEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `public_endpoint`.\n"]
    pub fn set_public_endpoint(
        self,
        v: impl Into<BlockAssignable<LightsailContainerServiceDeploymentVersionPublicEndpointEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().public_endpoint = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.public_endpoint = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<LightsailContainerServiceDeploymentVersionTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_endpoint` after provisioning.\n"]
    pub fn public_endpoint(
        &self,
    ) -> ListRef<LightsailContainerServiceDeploymentVersionPublicEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerServiceDeploymentVersionTimeoutsElRef {
        LightsailContainerServiceDeploymentVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for LightsailContainerServiceDeploymentVersion {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for LightsailContainerServiceDeploymentVersion {}
impl ToListMappable for LightsailContainerServiceDeploymentVersion {
    type O = ListRef<LightsailContainerServiceDeploymentVersionRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for LightsailContainerServiceDeploymentVersion_ {
    fn extract_resource_type(&self) -> String {
        "aws_lightsail_container_service_deployment_version".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildLightsailContainerServiceDeploymentVersion {
    pub tf_id: String,
    #[doc = ""]
    pub service_name: PrimField<String>,
}
impl BuildLightsailContainerServiceDeploymentVersion {
    pub fn build(self, stack: &mut Stack) -> LightsailContainerServiceDeploymentVersion {
        let out = LightsailContainerServiceDeploymentVersion(Rc::new(
            LightsailContainerServiceDeploymentVersion_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(LightsailContainerServiceDeploymentVersionData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    service_name: self.service_name,
                    container: core::default::Default::default(),
                    public_endpoint: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct LightsailContainerServiceDeploymentVersionRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServiceDeploymentVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl LightsailContainerServiceDeploymentVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.version", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `public_endpoint` after provisioning.\n"]
    pub fn public_endpoint(
        &self,
    ) -> ListRef<LightsailContainerServiceDeploymentVersionPublicEndpointElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.public_endpoint", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerServiceDeploymentVersionTimeoutsElRef {
        LightsailContainerServiceDeploymentVersionTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersionContainerEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    command: Option<ListField<PrimField<String>>>,
    container_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<RecField<PrimField<String>>>,
    image: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ports: Option<RecField<PrimField<String>>>,
}
impl LightsailContainerServiceDeploymentVersionContainerEl {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.command = Some(v.into());
        self
    }
    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.environment = Some(v.into());
        self
    }
    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.ports = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailContainerServiceDeploymentVersionContainerEl {
    type O = BlockAssignable<LightsailContainerServiceDeploymentVersionContainerEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServiceDeploymentVersionContainerEl {
    #[doc = ""]
    pub container_name: PrimField<String>,
    #[doc = ""]
    pub image: PrimField<String>,
}
impl BuildLightsailContainerServiceDeploymentVersionContainerEl {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersionContainerEl {
        LightsailContainerServiceDeploymentVersionContainerEl {
            command: core::default::Default::default(),
            container_name: self.container_name,
            environment: core::default::Default::default(),
            image: self.image,
            ports: core::default::Default::default(),
        }
    }
}
pub struct LightsailContainerServiceDeploymentVersionContainerElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServiceDeploymentVersionContainerElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailContainerServiceDeploymentVersionContainerElRef {
        LightsailContainerServiceDeploymentVersionContainerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServiceDeploymentVersionContainerElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }
    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }
    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }
    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}
#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    success_codes: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout_seconds: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold: Option<PrimField<f64>>,
}
impl LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl {
    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `interval_seconds`.\n"]
    pub fn set_interval_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `success_codes`.\n"]
    pub fn set_success_codes(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.success_codes = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout_seconds = Some(v.into());
        self
    }
    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unhealthy_threshold = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl {
    type O =
        BlockAssignable<LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl {}
impl BuildLightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl {
        LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl {
            healthy_threshold: core::default::Default::default(),
            interval_seconds: core::default::Default::default(),
            path: core::default::Default::default(),
            success_codes: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}
pub struct LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckElRef {
        LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.healthy_threshold", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `interval_seconds` after provisioning.\n"]
    pub fn interval_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.interval_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `success_codes` after provisioning.\n"]
    pub fn success_codes(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.success_codes", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.timeout_seconds", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unhealthy_threshold", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct LightsailContainerServiceDeploymentVersionPublicEndpointElDynamic {
    health_check: Option<
        DynamicBlock<LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl>,
    >,
}
#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersionPublicEndpointEl {
    container_name: PrimField<String>,
    container_port: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check:
        Option<Vec<LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl>>,
    dynamic: LightsailContainerServiceDeploymentVersionPublicEndpointElDynamic,
}
impl LightsailContainerServiceDeploymentVersionPublicEndpointEl {
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v: impl Into<
            BlockAssignable<
                LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.health_check = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for LightsailContainerServiceDeploymentVersionPublicEndpointEl {
    type O = BlockAssignable<LightsailContainerServiceDeploymentVersionPublicEndpointEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServiceDeploymentVersionPublicEndpointEl {
    #[doc = ""]
    pub container_name: PrimField<String>,
    #[doc = ""]
    pub container_port: PrimField<f64>,
}
impl BuildLightsailContainerServiceDeploymentVersionPublicEndpointEl {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersionPublicEndpointEl {
        LightsailContainerServiceDeploymentVersionPublicEndpointEl {
            container_name: self.container_name,
            container_port: self.container_port,
            health_check: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct LightsailContainerServiceDeploymentVersionPublicEndpointElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServiceDeploymentVersionPublicEndpointElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailContainerServiceDeploymentVersionPublicEndpointElRef {
        LightsailContainerServiceDeploymentVersionPublicEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServiceDeploymentVersionPublicEndpointElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_name", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_port", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(
        &self,
    ) -> ListRef<LightsailContainerServiceDeploymentVersionPublicEndpointElHealthCheckElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
}
#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersionTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl LightsailContainerServiceDeploymentVersionTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for LightsailContainerServiceDeploymentVersionTimeoutsEl {
    type O = BlockAssignable<LightsailContainerServiceDeploymentVersionTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildLightsailContainerServiceDeploymentVersionTimeoutsEl {}
impl BuildLightsailContainerServiceDeploymentVersionTimeoutsEl {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersionTimeoutsEl {
        LightsailContainerServiceDeploymentVersionTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct LightsailContainerServiceDeploymentVersionTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for LightsailContainerServiceDeploymentVersionTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> LightsailContainerServiceDeploymentVersionTimeoutsElRef {
        LightsailContainerServiceDeploymentVersionTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl LightsailContainerServiceDeploymentVersionTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize, Default)]
struct LightsailContainerServiceDeploymentVersionDynamic {
    container: Option<DynamicBlock<LightsailContainerServiceDeploymentVersionContainerEl>>,
    public_endpoint:
        Option<DynamicBlock<LightsailContainerServiceDeploymentVersionPublicEndpointEl>>,
}
