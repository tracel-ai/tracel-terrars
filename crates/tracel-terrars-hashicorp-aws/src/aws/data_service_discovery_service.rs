use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServiceDiscoveryServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    namespace_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataServiceDiscoveryService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataServiceDiscoveryServiceData>,
}

#[derive(Clone)]
pub struct DataServiceDiscoveryService(Rc<DataServiceDiscoveryService_>);

impl DataServiceDiscoveryService {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> ListRef<DataServiceDiscoveryServiceDnsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> ListRef<DataServiceDiscoveryServiceHealthCheckConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(&self) -> ListRef<DataServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Referable for DataServiceDiscoveryService {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataServiceDiscoveryService { }

impl ToListMappable for DataServiceDiscoveryService {
    type O = ListRef<DataServiceDiscoveryServiceRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataServiceDiscoveryService_ {
    fn extract_datasource_type(&self) -> String {
        "aws_service_discovery_service".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServiceDiscoveryService {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub namespace_id: PrimField<String>,
}

impl BuildDataServiceDiscoveryService {
    pub fn build(self, stack: &mut Stack) -> DataServiceDiscoveryService {
        let out = DataServiceDiscoveryService(Rc::new(DataServiceDiscoveryService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataServiceDiscoveryServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                namespace_id: self.namespace_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServiceDiscoveryServiceRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServiceDiscoveryServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataServiceDiscoveryServiceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_config` after provisioning.\n"]
    pub fn dns_config(&self) -> ListRef<DataServiceDiscoveryServiceDnsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_config` after provisioning.\n"]
    pub fn health_check_config(&self) -> ListRef<DataServiceDiscoveryServiceHealthCheckConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_custom_config` after provisioning.\n"]
    pub fn health_check_custom_config(&self) -> ListRef<DataServiceDiscoveryServiceHealthCheckCustomConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.health_check_custom_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    type O = BlockAssignable<DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {}

impl BuildDataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
    pub fn build(self) -> DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
        DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl {
            ttl: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
    fn new(shared: StackShared, base: String) -> DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
        DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef {
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

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceDnsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    dns_records: Option<ListField<DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    namespace_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    routing_policy: Option<PrimField<String>>,
}

impl DataServiceDiscoveryServiceDnsConfigEl {
    #[doc = "Set the field `dns_records`.\n"]
    pub fn set_dns_records(
        mut self,
        v: impl Into<ListField<DataServiceDiscoveryServiceDnsConfigElDnsRecordsEl>>,
    ) -> Self {
        self.dns_records = Some(v.into());
        self
    }

    #[doc = "Set the field `namespace_id`.\n"]
    pub fn set_namespace_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.namespace_id = Some(v.into());
        self
    }

    #[doc = "Set the field `routing_policy`.\n"]
    pub fn set_routing_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.routing_policy = Some(v.into());
        self
    }
}

impl ToListMappable for DataServiceDiscoveryServiceDnsConfigEl {
    type O = BlockAssignable<DataServiceDiscoveryServiceDnsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceDnsConfigEl {}

impl BuildDataServiceDiscoveryServiceDnsConfigEl {
    pub fn build(self) -> DataServiceDiscoveryServiceDnsConfigEl {
        DataServiceDiscoveryServiceDnsConfigEl {
            dns_records: core::default::Default::default(),
            namespace_id: core::default::Default::default(),
            routing_policy: core::default::Default::default(),
        }
    }
}

pub struct DataServiceDiscoveryServiceDnsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServiceDiscoveryServiceDnsConfigElRef {
    fn new(shared: StackShared, base: String) -> DataServiceDiscoveryServiceDnsConfigElRef {
        DataServiceDiscoveryServiceDnsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceDnsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_records` after provisioning.\n"]
    pub fn dns_records(&self) -> ListRef<DataServiceDiscoveryServiceDnsConfigElDnsRecordsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.dns_records", self.base))
    }

    #[doc = "Get a reference to the value of field `namespace_id` after provisioning.\n"]
    pub fn namespace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.namespace_id", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_policy` after provisioning.\n"]
    pub fn routing_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.routing_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceHealthCheckConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_path: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataServiceDiscoveryServiceHealthCheckConfigEl {
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

impl ToListMappable for DataServiceDiscoveryServiceHealthCheckConfigEl {
    type O = BlockAssignable<DataServiceDiscoveryServiceHealthCheckConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceHealthCheckConfigEl {}

impl BuildDataServiceDiscoveryServiceHealthCheckConfigEl {
    pub fn build(self) -> DataServiceDiscoveryServiceHealthCheckConfigEl {
        DataServiceDiscoveryServiceHealthCheckConfigEl {
            failure_threshold: core::default::Default::default(),
            resource_path: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataServiceDiscoveryServiceHealthCheckConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServiceDiscoveryServiceHealthCheckConfigElRef {
    fn new(shared: StackShared, base: String) -> DataServiceDiscoveryServiceHealthCheckConfigElRef {
        DataServiceDiscoveryServiceHealthCheckConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceHealthCheckConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_path` after provisioning.\n"]
    pub fn resource_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_path", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    failure_threshold: Option<PrimField<f64>>,
}

impl DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    #[doc = "Set the field `failure_threshold`.\n"]
    pub fn set_failure_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.failure_threshold = Some(v.into());
        self
    }
}

impl ToListMappable for DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    type O = BlockAssignable<DataServiceDiscoveryServiceHealthCheckCustomConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataServiceDiscoveryServiceHealthCheckCustomConfigEl {}

impl BuildDataServiceDiscoveryServiceHealthCheckCustomConfigEl {
    pub fn build(self) -> DataServiceDiscoveryServiceHealthCheckCustomConfigEl {
        DataServiceDiscoveryServiceHealthCheckCustomConfigEl { failure_threshold: core::default::Default::default() }
    }
}

pub struct DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn new(shared: StackShared, base: String) -> DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
        DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataServiceDiscoveryServiceHealthCheckCustomConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_threshold` after provisioning.\n"]
    pub fn failure_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_threshold", self.base))
    }
}
