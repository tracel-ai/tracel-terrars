use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVpcEndpointServiceData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_regions: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataVpcEndpointServiceFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataVpcEndpointServiceTimeoutsEl>,
    dynamic: DataVpcEndpointServiceDynamic,
}
struct DataVpcEndpointService_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcEndpointServiceData>,
}
#[derive(Clone)]
pub struct DataVpcEndpointService(Rc<DataVpcEndpointService_>);
impl DataVpcEndpointService {
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
    #[doc = "Set the field `service`.\n"]
    pub fn set_service(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service = Some(v.into());
        self
    }
    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_name = Some(v.into());
        self
    }
    #[doc = "Set the field `service_regions`.\n"]
    pub fn set_service_regions(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().service_regions = Some(v.into());
        self
    }
    #[doc = "Set the field `service_type`.\n"]
    pub fn set_service_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().service_type = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataVpcEndpointServiceFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataVpcEndpointServiceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `acceptance_required` after provisioning.\n"]
    pub fn acceptance_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.acceptance_required", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_endpoint_dns_names` after provisioning.\n"]
    pub fn base_endpoint_dns_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.base_endpoint_dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `manages_vpc_endpoints` after provisioning.\n"]
    pub fn manages_vpc_endpoints(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manages_vpc_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_dns_names` after provisioning.\n"]
    pub fn private_dns_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.private_dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `service_region` after provisioning.\n"]
    pub fn service_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_regions` after provisioning.\n"]
    pub fn service_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.service_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_type` after provisioning.\n"]
    pub fn service_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_ip_address_types` after provisioning.\n"]
    pub fn supported_ip_address_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_ip_address_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_policy_supported` after provisioning.\n"]
    pub fn vpc_endpoint_policy_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_policy_supported", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcEndpointServiceTimeoutsElRef {
        DataVpcEndpointServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataVpcEndpointService {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVpcEndpointService {}
impl ToListMappable for DataVpcEndpointService {
    type O = ListRef<DataVpcEndpointServiceRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVpcEndpointService_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpc_endpoint_service".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVpcEndpointService {
    pub tf_id: String,
}
impl BuildDataVpcEndpointService {
    pub fn build(self, stack: &mut Stack) -> DataVpcEndpointService {
        let out = DataVpcEndpointService(Rc::new(DataVpcEndpointService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcEndpointServiceData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                service: core::default::Default::default(),
                service_name: core::default::Default::default(),
                service_regions: core::default::Default::default(),
                service_type: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVpcEndpointServiceRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcEndpointServiceRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVpcEndpointServiceRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `acceptance_required` after provisioning.\n"]
    pub fn acceptance_required(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.acceptance_required", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `base_endpoint_dns_names` after provisioning.\n"]
    pub fn base_endpoint_dns_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.base_endpoint_dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `manages_vpc_endpoints` after provisioning.\n"]
    pub fn manages_vpc_endpoints(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.manages_vpc_endpoints", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owner", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `private_dns_names` after provisioning.\n"]
    pub fn private_dns_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.private_dns_names", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `service_region` after provisioning.\n"]
    pub fn service_region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_regions` after provisioning.\n"]
    pub fn service_regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.service_regions", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `service_type` after provisioning.\n"]
    pub fn service_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.service_type", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `supported_ip_address_types` after provisioning.\n"]
    pub fn supported_ip_address_types(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_ip_address_types", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_policy_supported` after provisioning.\n"]
    pub fn vpc_endpoint_policy_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_policy_supported", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcEndpointServiceTimeoutsElRef {
        DataVpcEndpointServiceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataVpcEndpointServiceFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}
impl DataVpcEndpointServiceFilterEl {}
impl ToListMappable for DataVpcEndpointServiceFilterEl {
    type O = BlockAssignable<DataVpcEndpointServiceFilterEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpcEndpointServiceFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}
impl BuildDataVpcEndpointServiceFilterEl {
    pub fn build(self) -> DataVpcEndpointServiceFilterEl {
        DataVpcEndpointServiceFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}
pub struct DataVpcEndpointServiceFilterElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcEndpointServiceFilterElRef {
    fn new(shared: StackShared, base: String) -> DataVpcEndpointServiceFilterElRef {
        DataVpcEndpointServiceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpcEndpointServiceFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
#[derive(Serialize)]
pub struct DataVpcEndpointServiceTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataVpcEndpointServiceTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpcEndpointServiceTimeoutsEl {
    type O = BlockAssignable<DataVpcEndpointServiceTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpcEndpointServiceTimeoutsEl {}
impl BuildDataVpcEndpointServiceTimeoutsEl {
    pub fn build(self) -> DataVpcEndpointServiceTimeoutsEl {
        DataVpcEndpointServiceTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataVpcEndpointServiceTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcEndpointServiceTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcEndpointServiceTimeoutsElRef {
        DataVpcEndpointServiceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpcEndpointServiceTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataVpcEndpointServiceDynamic {
    filter: Option<DynamicBlock<DataVpcEndpointServiceFilterEl>>,
}
