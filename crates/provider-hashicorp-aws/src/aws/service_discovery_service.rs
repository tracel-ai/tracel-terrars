use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ServiceDiscoveryServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_config: Option<Vec<ServiceDiscoveryServiceDnsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_config: Option<Vec<ServiceDiscoveryServiceHealthCheckConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_custom_config: Option<Vec<ServiceDiscoveryServiceHealthCheckCustomConfigEl>>,
    dynamic: ServiceDiscoveryServiceDynamic,
}

struct ServiceDiscoveryService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ServiceDiscoveryServiceData>,
}

#[derive(Clone)]
pub struct ServiceDiscoveryService(Rc<ServiceDiscoveryService_>);

impl ServiceDiscoveryService {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_id`.\n"]
    pub fn set_namespace_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().namespace_id = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc = "Set the field `dns_config`.\n"]
    pub fn set_dns_config(
        self,
        v: impl Into<BlockAssignable<ServiceDiscoveryServiceDnsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dns_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dns_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `health_check_config`.\n"]
    pub fn set_health_check_config(
        self,
        v: impl Into<BlockAssignable<ServiceDiscoveryServiceHealthCheckConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `health_check_custom_config`.\n"]
    pub fn set_health_check_custom_config(
        self,
        v: impl Into<BlockAssignable<ServiceDiscoveryServiceHealthCheckCustomConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check_custom_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check_custom_config = Some(d);
            }
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> ListRef<ServiceDiscoveryServiceDnsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> ListRef<ServiceDiscoveryServiceHealthCheckConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(
        &self,
    ) -> ListRef<ServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check_custom_config", self.extract_ref()),
        )
    }
}

impl Referable for ServiceDiscoveryService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for ServiceDiscoveryService {}

impl ToListMappable for ServiceDiscoveryService {
    type O = ListRef<ServiceDiscoveryServiceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ServiceDiscoveryService_ {
    fn extract_resource_type(&self) -> String {
        "aws_service_discovery_service".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildServiceDiscoveryService {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildServiceDiscoveryService {
    pub fn build(self, stack: &mut Stack) -> ServiceDiscoveryService {
        let out = ServiceDiscoveryService(Rc::new(ServiceDiscoveryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ServiceDiscoveryServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                description: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                namespace_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                dns_config: core::default::Default::default(),
                health_check_config: core::default::Default::default(),
                health_check_custom_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ServiceDiscoveryServiceRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceDiscoveryServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl ServiceDiscoveryServiceRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.namespace_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> ListRef<ServiceDiscoveryServiceDnsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dns_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> ListRef<ServiceDiscoveryServiceHealthCheckConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(
        &self,
    ) -> ListRef<ServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check_custom_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    ttl: PrimField<f64>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {}

impl ToListMappable for ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    type O = BlockAssignable<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    #[doc = ""]
    pub ttl: PrimField<f64>,
    #[doc = ""]
    pub type_: PrimField<String>,
}

impl BuildServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    pub fn build(self) -> ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
        ServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
            ttl: self.ttl,
            type_: self.type_,
        }
    }
}

pub struct ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    fn new(shared: StackShared, base: String) -> ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
        ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct ServiceDiscoveryServiceDnsConfigElDynamic {
    dns_records: Option<DynamicBlock<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceDnsConfigEl {
    namespace_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_records: Option<Vec<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    dynamic: ServiceDiscoveryServiceDnsConfigElDynamic,
}

impl ServiceDiscoveryServiceDnsConfigEl {
    #[doc = "Set the field `routing_policy`.\n"]
    pub fn set_routing_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.routing_policy = Some(v.into());
        self
    }

    #[doc = "Set the field `dns_records`.\n"]
    pub fn set_dns_records(
        mut self,
        v: impl Into<BlockAssignable<ServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.dns_records = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_records = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for ServiceDiscoveryServiceDnsConfigEl {
    type O = BlockAssignable<ServiceDiscoveryServiceDnsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceDnsConfigEl {
    #[doc = ""]
    pub namespace_id: PrimField<String>,
}

impl BuildServiceDiscoveryServiceDnsConfigEl {
    pub fn build(self) -> ServiceDiscoveryServiceDnsConfigEl {
        ServiceDiscoveryServiceDnsConfigEl {
            namespace_id: self.namespace_id,
            routing_policy: core::default::Default::default(),
            dns_records: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct ServiceDiscoveryServiceDnsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceDiscoveryServiceDnsConfigElRef {
    fn new(shared: StackShared, base: String) -> ServiceDiscoveryServiceDnsConfigElRef {
        ServiceDiscoveryServiceDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceDnsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.routing_policy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> ListRef<ServiceDiscoveryServiceDnsConfigElDnsRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_records", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceHealthCheckConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_path: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl ServiceDiscoveryServiceHealthCheckConfigEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_path`.\n"]
    pub fn set_resource_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_path = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceDiscoveryServiceHealthCheckConfigEl {
    type O = BlockAssignable<ServiceDiscoveryServiceHealthCheckConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceHealthCheckConfigEl {}

impl BuildServiceDiscoveryServiceHealthCheckConfigEl {
    pub fn build(self) -> ServiceDiscoveryServiceHealthCheckConfigEl {
        ServiceDiscoveryServiceHealthCheckConfigEl {
            failure_threshold: core::default::Default::default(),
            resource_path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct ServiceDiscoveryServiceHealthCheckConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceDiscoveryServiceHealthCheckConfigElRef {
    fn new(shared: StackShared, base: String) -> ServiceDiscoveryServiceHealthCheckConfigElRef {
        ServiceDiscoveryServiceHealthCheckConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceHealthCheckConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_path` after provisioning.\n"]
    pub fn resource_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_path", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct ServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
}

impl ServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
}

impl ToListMappable for ServiceDiscoveryServiceHealthCheckCustomConfigEl {
    type O = BlockAssignable<ServiceDiscoveryServiceHealthCheckCustomConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildServiceDiscoveryServiceHealthCheckCustomConfigEl {}

impl BuildServiceDiscoveryServiceHealthCheckCustomConfigEl {
    pub fn build(self) -> ServiceDiscoveryServiceHealthCheckCustomConfigEl {
        ServiceDiscoveryServiceHealthCheckCustomConfigEl {
            failure_threshold: core::default::Default::default(),
        }
    }
}

pub struct ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
        ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.failure_threshold", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct ServiceDiscoveryServiceDynamic {
    dns_config: Option<DynamicBlock<ServiceDiscoveryServiceDnsConfigEl>>,
    health_check_config: Option<DynamicBlock<ServiceDiscoveryServiceHealthCheckConfigEl>>,
    health_check_custom_config:
        Option<DynamicBlock<ServiceDiscoveryServiceHealthCheckCustomConfigEl>>,
}
