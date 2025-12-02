use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ApprunnerServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling_configuration_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    service_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<ApprunnerServiceEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    health_check_configuration: Option<Vec<ApprunnerServiceHealthCheckConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_configuration: Option<Vec<ApprunnerServiceInstanceConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<ApprunnerServiceNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observability_configuration: Option<Vec<ApprunnerServiceObservabilityConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_configuration: Option<Vec<ApprunnerServiceSourceConfigurationEl>>,
    dynamic: ApprunnerServiceDynamic,
}
struct ApprunnerService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApprunnerServiceData>,
}
#[derive(Clone)]
pub struct ApprunnerService(Rc<ApprunnerService_>);
impl ApprunnerService {
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
    #[doc = "Set the field `auto_scaling_configuration_arn`.\n"]
    pub fn set_auto_scaling_configuration_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().auto_scaling_configuration_arn = Some(v.into());
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
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<ApprunnerServiceEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `health_check_configuration`.\n"]
    pub fn set_health_check_configuration(
        self,
        v: impl Into<BlockAssignable<ApprunnerServiceHealthCheckConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().health_check_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.health_check_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `instance_configuration`.\n"]
    pub fn set_instance_configuration(
        self,
        v: impl Into<BlockAssignable<ApprunnerServiceInstanceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<ApprunnerServiceNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `observability_configuration`.\n"]
    pub fn set_observability_configuration(
        self,
        v: impl Into<BlockAssignable<ApprunnerServiceObservabilityConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().observability_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.observability_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_configuration`.\n"]
    pub fn set_source_configuration(
        self,
        v: impl Into<BlockAssignable<ApprunnerServiceSourceConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `auto_scaling_configuration_arn` after provisioning.\n"]
    pub fn auto_scaling_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_scaling_configuration_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_url` after provisioning.\n"]
    pub fn service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_check_configuration` after provisioning.\n"]
    pub fn health_check_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceHealthCheckConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> ListRef<ApprunnerServiceInstanceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<ApprunnerServiceNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observability_configuration` after provisioning.\n"]
    pub fn observability_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceObservabilityConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.observability_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(&self) -> ListRef<ApprunnerServiceSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_configuration", self.extract_ref()),
        )
    }
}
impl Referable for ApprunnerService {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ApprunnerService {}
impl ToListMappable for ApprunnerService {
    type O = ListRef<ApprunnerServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ApprunnerService_ {
    fn extract_resource_type(&self) -> String {
        "aws_apprunner_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildApprunnerService {
    pub tf_id: String,
    #[doc = ""]
    pub service_name: PrimField<String>,
}
impl BuildApprunnerService {
    pub fn build(self, stack: &mut Stack) -> ApprunnerService {
        let out = ApprunnerService(Rc::new(ApprunnerService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApprunnerServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                auto_scaling_configuration_arn: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                service_name: self.service_name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                health_check_configuration: core::default::Default::default(),
                instance_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                observability_configuration: core::default::Default::default(),
                source_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ApprunnerServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ApprunnerServiceRef {
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
    #[doc = "Get a reference to the value of field `auto_scaling_configuration_arn` after provisioning.\n"]
    pub fn auto_scaling_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_scaling_configuration_arn", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_url` after provisioning.\n"]
    pub fn service_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_url", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.status", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceEncryptionConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.encryption_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `health_check_configuration` after provisioning.\n"]
    pub fn health_check_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceHealthCheckConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.health_check_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `instance_configuration` after provisioning.\n"]
    pub fn instance_configuration(&self) -> ListRef<ApprunnerServiceInstanceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.instance_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<ApprunnerServiceNetworkConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.network_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `observability_configuration` after provisioning.\n"]
    pub fn observability_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceObservabilityConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.observability_configuration", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `source_configuration` after provisioning.\n"]
    pub fn source_configuration(&self) -> ListRef<ApprunnerServiceSourceConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_configuration", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceEncryptionConfigurationEl {
    kms_key: PrimField<String>,
}
impl ApprunnerServiceEncryptionConfigurationEl {}
impl ToListMappable for ApprunnerServiceEncryptionConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceEncryptionConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceEncryptionConfigurationEl {
    #[doc = ""]
    pub kms_key: PrimField<String>,
}
impl BuildApprunnerServiceEncryptionConfigurationEl {
    pub fn build(self) -> ApprunnerServiceEncryptionConfigurationEl {
        ApprunnerServiceEncryptionConfigurationEl {
            kms_key: self.kms_key,
        }
    }
}
pub struct ApprunnerServiceEncryptionConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApprunnerServiceEncryptionConfigurationElRef {
        ApprunnerServiceEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kms_key` after provisioning.\n"]
    pub fn kms_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key", self.base))
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceHealthCheckConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    healthy_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interval: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unhealthy_threshold: Option<PrimField<f64>>,
}
impl ApprunnerServiceHealthCheckConfigurationEl {
    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.healthy_threshold = Some(v.into());
        self
    }
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.interval = Some(v.into());
        self
    }
    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.path = Some(v.into());
        self
    }
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.protocol = Some(v.into());
        self
    }
    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.timeout = Some(v.into());
        self
    }
    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.unhealthy_threshold = Some(v.into());
        self
    }
}
impl ToListMappable for ApprunnerServiceHealthCheckConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceHealthCheckConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceHealthCheckConfigurationEl {}
impl BuildApprunnerServiceHealthCheckConfigurationEl {
    pub fn build(self) -> ApprunnerServiceHealthCheckConfigurationEl {
        ApprunnerServiceHealthCheckConfigurationEl {
            healthy_threshold: core::default::Default::default(),
            interval: core::default::Default::default(),
            path: core::default::Default::default(),
            protocol: core::default::Default::default(),
            timeout: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}
pub struct ApprunnerServiceHealthCheckConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceHealthCheckConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApprunnerServiceHealthCheckConfigurationElRef {
        ApprunnerServiceHealthCheckConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceHealthCheckConfigurationElRef {
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
    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.unhealthy_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceInstanceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory: Option<PrimField<String>>,
}
impl ApprunnerServiceInstanceConfigurationEl {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cpu = Some(v.into());
        self
    }
    #[doc = "Set the field `instance_role_arn`.\n"]
    pub fn set_instance_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.instance_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.memory = Some(v.into());
        self
    }
}
impl ToListMappable for ApprunnerServiceInstanceConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceInstanceConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceInstanceConfigurationEl {}
impl BuildApprunnerServiceInstanceConfigurationEl {
    pub fn build(self) -> ApprunnerServiceInstanceConfigurationEl {
        ApprunnerServiceInstanceConfigurationEl {
            cpu: core::default::Default::default(),
            instance_role_arn: core::default::Default::default(),
            memory: core::default::Default::default(),
        }
    }
}
pub struct ApprunnerServiceInstanceConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceInstanceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApprunnerServiceInstanceConfigurationElRef {
        ApprunnerServiceInstanceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceInstanceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }
    #[doc = "Get a reference to the value of field `instance_role_arn` after provisioning.\n"]
    pub fn instance_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_connector_arn: Option<PrimField<String>>,
}
impl ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    #[doc = "Set the field `egress_type`.\n"]
    pub fn set_egress_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.egress_type = Some(v.into());
        self
    }
    #[doc = "Set the field `vpc_connector_arn`.\n"]
    pub fn set_vpc_connector_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.vpc_connector_arn = Some(v.into());
        self
    }
}
impl ToListMappable for ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceNetworkConfigurationElEgressConfigurationEl {}
impl BuildApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
    pub fn build(self) -> ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
        ApprunnerServiceNetworkConfigurationElEgressConfigurationEl {
            egress_type: core::default::Default::default(),
            vpc_connector_arn: core::default::Default::default(),
        }
    }
}
pub struct ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
        ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `egress_type` after provisioning.\n"]
    pub fn egress_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.egress_type", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_connector_arn` after provisioning.\n"]
    pub fn vpc_connector_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_connector_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_publicly_accessible: Option<PrimField<bool>>,
}
impl ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    #[doc = "Set the field `is_publicly_accessible`.\n"]
    pub fn set_is_publicly_accessible(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_publicly_accessible = Some(v.into());
        self
    }
}
impl ToListMappable for ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceNetworkConfigurationElIngressConfigurationEl {}
impl BuildApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
    pub fn build(self) -> ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
        ApprunnerServiceNetworkConfigurationElIngressConfigurationEl {
            is_publicly_accessible: core::default::Default::default(),
        }
    }
}
pub struct ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
        ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `is_publicly_accessible` after provisioning.\n"]
    pub fn is_publicly_accessible(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.is_publicly_accessible", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApprunnerServiceNetworkConfigurationElDynamic {
    egress_configuration:
        Option<DynamicBlock<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>>,
    ingress_configuration:
        Option<DynamicBlock<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>>,
}
#[derive(Serialize)]
pub struct ApprunnerServiceNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    egress_configuration: Option<Vec<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingress_configuration:
        Option<Vec<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>>,
    dynamic: ApprunnerServiceNetworkConfigurationElDynamic,
}
impl ApprunnerServiceNetworkConfigurationEl {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip_address_type = Some(v.into());
        self
    }
    #[doc = "Set the field `egress_configuration`.\n"]
    pub fn set_egress_configuration(
        mut self,
        v: impl Into<BlockAssignable<ApprunnerServiceNetworkConfigurationElEgressConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.egress_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.egress_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ingress_configuration`.\n"]
    pub fn set_ingress_configuration(
        mut self,
        v: impl Into<BlockAssignable<ApprunnerServiceNetworkConfigurationElIngressConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ingress_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ingress_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApprunnerServiceNetworkConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceNetworkConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceNetworkConfigurationEl {}
impl BuildApprunnerServiceNetworkConfigurationEl {
    pub fn build(self) -> ApprunnerServiceNetworkConfigurationEl {
        ApprunnerServiceNetworkConfigurationEl {
            ip_address_type: core::default::Default::default(),
            egress_configuration: core::default::Default::default(),
            ingress_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApprunnerServiceNetworkConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApprunnerServiceNetworkConfigurationElRef {
        ApprunnerServiceNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ip_address_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `egress_configuration` after provisioning.\n"]
    pub fn egress_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceNetworkConfigurationElEgressConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.egress_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ingress_configuration` after provisioning.\n"]
    pub fn ingress_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceNetworkConfigurationElIngressConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ingress_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceObservabilityConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    observability_configuration_arn: Option<PrimField<String>>,
    observability_enabled: PrimField<bool>,
}
impl ApprunnerServiceObservabilityConfigurationEl {
    #[doc = "Set the field `observability_configuration_arn`.\n"]
    pub fn set_observability_configuration_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.observability_configuration_arn = Some(v.into());
        self
    }
}
impl ToListMappable for ApprunnerServiceObservabilityConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceObservabilityConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceObservabilityConfigurationEl {
    #[doc = ""]
    pub observability_enabled: PrimField<bool>,
}
impl BuildApprunnerServiceObservabilityConfigurationEl {
    pub fn build(self) -> ApprunnerServiceObservabilityConfigurationEl {
        ApprunnerServiceObservabilityConfigurationEl {
            observability_configuration_arn: core::default::Default::default(),
            observability_enabled: self.observability_enabled,
        }
    }
}
pub struct ApprunnerServiceObservabilityConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceObservabilityConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApprunnerServiceObservabilityConfigurationElRef {
        ApprunnerServiceObservabilityConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceObservabilityConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `observability_configuration_arn` after provisioning.\n"]
    pub fn observability_configuration_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observability_configuration_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `observability_enabled` after provisioning.\n"]
    pub fn observability_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.observability_enabled", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_role_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    connection_arn: Option<PrimField<String>>,
}
impl ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    #[doc = "Set the field `access_role_arn`.\n"]
    pub fn set_access_role_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.access_role_arn = Some(v.into());
        self
    }
    #[doc = "Set the field `connection_arn`.\n"]
    pub fn set_connection_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.connection_arn = Some(v.into());
        self
    }
}
impl ToListMappable for ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {}
impl BuildApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
        ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl {
            access_role_arn: core::default::Default::default(),
            connection_arn: core::default::Default::default(),
        }
    }
}
pub struct ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
        ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_role_arn` after provisioning.\n"]
    pub fn access_role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_role_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `connection_arn` after provisioning.\n"]
    pub fn connection_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.connection_arn", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    build_command: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    runtime: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_environment_secrets: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_command: Option<PrimField<String>>,
}
impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl { # [doc = "Set the field `build_command`.\n"] pub fn set_build_command (mut self , v : impl Into < PrimField < String > >) -> Self { self . build_command = Some (v . into ()) ; self } # [doc = "Set the field `port`.\n"] pub fn set_port (mut self , v : impl Into < PrimField < String > >) -> Self { self . port = Some (v . into ()) ; self } # [doc = "Set the field `runtime_environment_secrets`.\n"] pub fn set_runtime_environment_secrets (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . runtime_environment_secrets = Some (v . into ()) ; self } # [doc = "Set the field `runtime_environment_variables`.\n"] pub fn set_runtime_environment_variables (mut self , v : impl Into < RecField < PrimField < String > > >) -> Self { self . runtime_environment_variables = Some (v . into ()) ; self } # [doc = "Set the field `start_command`.\n"] pub fn set_start_command (mut self , v : impl Into < PrimField < String > >) -> Self { self . start_command = Some (v . into ()) ; self } }
impl ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl { type O = BlockAssignable < ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl
{
    #[doc = ""]
    pub runtime: PrimField<String>,
}
impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl { pub fn build (self) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl { ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl { build_command : core :: default :: Default :: default () , port : core :: default :: Default :: default () , runtime : self . runtime , runtime_environment_secrets : core :: default :: Default :: default () , runtime_environment_variables : core :: default :: Default :: default () , start_command : core :: default :: Default :: default () , } } }
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef { fn new (shared : StackShared , base : String) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef { ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef { shared : shared , base : base . to_string () , } } }
impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `build_command` after provisioning.\n"] pub fn build_command (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.build_command" , self . base)) } # [doc = "Get a reference to the value of field `port` after provisioning.\n"] pub fn port (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.port" , self . base)) } # [doc = "Get a reference to the value of field `runtime` after provisioning.\n"] pub fn runtime (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.runtime" , self . base)) } # [doc = "Get a reference to the value of field `runtime_environment_secrets` after provisioning.\n"] pub fn runtime_environment_secrets (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.runtime_environment_secrets" , self . base)) } # [doc = "Get a reference to the value of field `runtime_environment_variables` after provisioning.\n"] pub fn runtime_environment_variables (& self) -> RecRef < PrimExpr < String > > { RecRef :: new (self . shared () . clone () , format ! ("{}.runtime_environment_variables" , self . base)) } # [doc = "Get a reference to the value of field `start_command` after provisioning.\n"] pub fn start_command (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.start_command" , self . base)) } }
#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElDynamic { code_configuration_values : Option < DynamicBlock < ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl >> , }
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl { configuration_source : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] code_configuration_values : Option < Vec < ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl > > , dynamic : ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElDynamic , }
impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    #[doc = "Set the field `code_configuration_values`.\n"]
    pub fn set_code_configuration_values(
        mut self,
        v : impl Into < BlockAssignable < ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_configuration_values = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_configuration_values = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    type O =
        BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    #[doc = ""]
    pub configuration_source: PrimField<String>,
}
impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl {
            configuration_source: self.configuration_source,
            code_configuration_values: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `configuration_source` after provisioning.\n"]
    pub fn configuration_source(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.configuration_source", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `code_configuration_values` after provisioning.\n"]    pub fn code_configuration_values (& self) -> ListRef < ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElCodeConfigurationValuesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_configuration_values", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
    value: PrimField<String>,
}
impl ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {}
impl ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    type O =
        BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    #[doc = ""]
    pub type_: PrimField<String>,
    #[doc = ""]
    pub value: PrimField<String>,
}
impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl {
            type_: self.type_,
            value: self.value,
        }
    }
}
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElCodeRepositoryElDynamic {
    code_configuration: Option<
        DynamicBlock<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl>,
    >,
    source_code_version: Option<
        DynamicBlock<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl>,
    >,
}
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    repository_url: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_directory: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_configuration:
        Option<Vec<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_code_version:
        Option<Vec<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl>>,
    dynamic: ApprunnerServiceSourceConfigurationElCodeRepositoryElDynamic,
}
impl ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    #[doc = "Set the field `source_directory`.\n"]
    pub fn set_source_directory(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_directory = Some(v.into());
        self
    }
    #[doc = "Set the field `code_configuration`.\n"]
    pub fn set_code_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source_code_version`.\n"]
    pub fn set_source_code_version(
        mut self,
        v: impl Into<
            BlockAssignable<
                ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_code_version = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_code_version = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    type O = BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    #[doc = ""]
    pub repository_url: PrimField<String>,
}
impl BuildApprunnerServiceSourceConfigurationElCodeRepositoryEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
        ApprunnerServiceSourceConfigurationElCodeRepositoryEl {
            repository_url: self.repository_url,
            source_directory: core::default::Default::default(),
            code_configuration: core::default::Default::default(),
            source_code_version: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
        ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceSourceConfigurationElCodeRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `repository_url` after provisioning.\n"]
    pub fn repository_url(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.repository_url", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_directory` after provisioning.\n"]
    pub fn source_directory(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.source_directory", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `code_configuration` after provisioning.\n"]
    pub fn code_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceSourceConfigurationElCodeRepositoryElCodeConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `source_code_version` after provisioning.\n"]
    pub fn source_code_version(
        &self,
    ) -> ListRef<ApprunnerServiceSourceConfigurationElCodeRepositoryElSourceCodeVersionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_code_version", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_environment_secrets: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    runtime_environment_variables: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_command: Option<PrimField<String>>,
}
impl ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.port = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_environment_secrets`.\n"]
    pub fn set_runtime_environment_secrets(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.runtime_environment_secrets = Some(v.into());
        self
    }
    #[doc = "Set the field `runtime_environment_variables`.\n"]
    pub fn set_runtime_environment_variables(
        mut self,
        v: impl Into<RecField<PrimField<String>>>,
    ) -> Self {
        self.runtime_environment_variables = Some(v.into());
        self
    }
    #[doc = "Set the field `start_command`.\n"]
    pub fn set_start_command(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.start_command = Some(v.into());
        self
    }
}
impl ToListMappable for ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    type O =
        BlockAssignable<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {}
impl BuildApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
    pub fn build(
        self,
    ) -> ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
        ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl {
            port: core::default::Default::default(),
            runtime_environment_secrets: core::default::Default::default(),
            runtime_environment_variables: core::default::Default::default(),
            start_command: core::default::Default::default(),
        }
    }
}
pub struct ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
        ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
    #[doc = "Get a reference to the value of field `runtime_environment_secrets` after provisioning.\n"]
    pub fn runtime_environment_secrets(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.runtime_environment_secrets", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `runtime_environment_variables` after provisioning.\n"]
    pub fn runtime_environment_variables(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.runtime_environment_variables", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `start_command` after provisioning.\n"]
    pub fn start_command(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.start_command", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElImageRepositoryElDynamic {
    image_configuration: Option<
        DynamicBlock<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl>,
    >,
}
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationElImageRepositoryEl {
    image_identifier: PrimField<String>,
    image_repository_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_configuration:
        Option<Vec<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl>>,
    dynamic: ApprunnerServiceSourceConfigurationElImageRepositoryElDynamic,
}
impl ApprunnerServiceSourceConfigurationElImageRepositoryEl {
    #[doc = "Set the field `image_configuration`.\n"]
    pub fn set_image_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<
                ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApprunnerServiceSourceConfigurationElImageRepositoryEl {
    type O = BlockAssignable<ApprunnerServiceSourceConfigurationElImageRepositoryEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceSourceConfigurationElImageRepositoryEl {
    #[doc = ""]
    pub image_identifier: PrimField<String>,
    #[doc = ""]
    pub image_repository_type: PrimField<String>,
}
impl BuildApprunnerServiceSourceConfigurationElImageRepositoryEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationElImageRepositoryEl {
        ApprunnerServiceSourceConfigurationElImageRepositoryEl {
            image_identifier: self.image_identifier,
            image_repository_type: self.image_repository_type,
            image_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
        ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceSourceConfigurationElImageRepositoryElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `image_identifier` after provisioning.\n"]
    pub fn image_identifier(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_identifier", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_repository_type` after provisioning.\n"]
    pub fn image_repository_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.image_repository_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_configuration` after provisioning.\n"]
    pub fn image_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceSourceConfigurationElImageRepositoryElImageConfigurationElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApprunnerServiceSourceConfigurationElDynamic {
    authentication_configuration:
        Option<DynamicBlock<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>>,
    code_repository: Option<DynamicBlock<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>>,
    image_repository: Option<DynamicBlock<ApprunnerServiceSourceConfigurationElImageRepositoryEl>>,
}
#[derive(Serialize)]
pub struct ApprunnerServiceSourceConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_deployments_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authentication_configuration:
        Option<Vec<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code_repository: Option<Vec<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_repository: Option<Vec<ApprunnerServiceSourceConfigurationElImageRepositoryEl>>,
    dynamic: ApprunnerServiceSourceConfigurationElDynamic,
}
impl ApprunnerServiceSourceConfigurationEl {
    #[doc = "Set the field `auto_deployments_enabled`.\n"]
    pub fn set_auto_deployments_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_deployments_enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `authentication_configuration`.\n"]
    pub fn set_authentication_configuration(
        mut self,
        v: impl Into<
            BlockAssignable<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.authentication_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.authentication_configuration = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `code_repository`.\n"]
    pub fn set_code_repository(
        mut self,
        v: impl Into<BlockAssignable<ApprunnerServiceSourceConfigurationElCodeRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.code_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `image_repository`.\n"]
    pub fn set_image_repository(
        mut self,
        v: impl Into<BlockAssignable<ApprunnerServiceSourceConfigurationElImageRepositoryEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.image_repository = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.image_repository = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ApprunnerServiceSourceConfigurationEl {
    type O = BlockAssignable<ApprunnerServiceSourceConfigurationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildApprunnerServiceSourceConfigurationEl {}
impl BuildApprunnerServiceSourceConfigurationEl {
    pub fn build(self) -> ApprunnerServiceSourceConfigurationEl {
        ApprunnerServiceSourceConfigurationEl {
            auto_deployments_enabled: core::default::Default::default(),
            authentication_configuration: core::default::Default::default(),
            code_repository: core::default::Default::default(),
            image_repository: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ApprunnerServiceSourceConfigurationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ApprunnerServiceSourceConfigurationElRef {
    fn new(shared: StackShared, base: String) -> ApprunnerServiceSourceConfigurationElRef {
        ApprunnerServiceSourceConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ApprunnerServiceSourceConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_deployments_enabled` after provisioning.\n"]
    pub fn auto_deployments_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_deployments_enabled", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(
        &self,
    ) -> ListRef<ApprunnerServiceSourceConfigurationElAuthenticationConfigurationElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.authentication_configuration", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `code_repository` after provisioning.\n"]
    pub fn code_repository(
        &self,
    ) -> ListRef<ApprunnerServiceSourceConfigurationElCodeRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.code_repository", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `image_repository` after provisioning.\n"]
    pub fn image_repository(
        &self,
    ) -> ListRef<ApprunnerServiceSourceConfigurationElImageRepositoryElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.image_repository", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ApprunnerServiceDynamic {
    encryption_configuration: Option<DynamicBlock<ApprunnerServiceEncryptionConfigurationEl>>,
    health_check_configuration: Option<DynamicBlock<ApprunnerServiceHealthCheckConfigurationEl>>,
    instance_configuration: Option<DynamicBlock<ApprunnerServiceInstanceConfigurationEl>>,
    network_configuration: Option<DynamicBlock<ApprunnerServiceNetworkConfigurationEl>>,
    observability_configuration: Option<DynamicBlock<ApprunnerServiceObservabilityConfigurationEl>>,
    source_configuration: Option<DynamicBlock<ApprunnerServiceSourceConfigurationEl>>,
}
