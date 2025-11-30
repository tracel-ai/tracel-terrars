use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataMqBrokerInstanceTypeOfferingsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
}

struct DataMqBrokerInstanceTypeOfferings_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMqBrokerInstanceTypeOfferingsData>,
}

#[derive(Clone)]
pub struct DataMqBrokerInstanceTypeOfferings(Rc<DataMqBrokerInstanceTypeOfferings_>);

impl DataMqBrokerInstanceTypeOfferings {
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

    #[doc = "Set the field `engine_type`.\n"]
    pub fn set_engine_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().engine_type = Some(v.into());
        self
    }

    #[doc = "Set the field `host_instance_type`.\n"]
    pub fn set_host_instance_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().host_instance_type = Some(v.into());
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

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().storage_type = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `broker_instance_options` after provisioning.\n"]
    pub fn broker_instance_options(
        &self,
    ) -> ListRef<DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.broker_instance_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_instance_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
        )
    }
}

impl Referable for DataMqBrokerInstanceTypeOfferings {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataMqBrokerInstanceTypeOfferings {}

impl ToListMappable for DataMqBrokerInstanceTypeOfferings {
    type O = ListRef<DataMqBrokerInstanceTypeOfferingsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataMqBrokerInstanceTypeOfferings_ {
    fn extract_datasource_type(&self) -> String {
        "aws_mq_broker_instance_type_offerings".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMqBrokerInstanceTypeOfferings {
    pub tf_id: String,
}

impl BuildDataMqBrokerInstanceTypeOfferings {
    pub fn build(self, stack: &mut Stack) -> DataMqBrokerInstanceTypeOfferings {
        let out = DataMqBrokerInstanceTypeOfferings(Rc::new(DataMqBrokerInstanceTypeOfferings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMqBrokerInstanceTypeOfferingsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                engine_type: core::default::Default::default(),
                host_instance_type: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                storage_type: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMqBrokerInstanceTypeOfferingsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerInstanceTypeOfferingsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataMqBrokerInstanceTypeOfferingsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `broker_instance_options` after provisioning.\n"]
    pub fn broker_instance_options(
        &self,
    ) -> ListRef<DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.broker_instance_options", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.engine_type", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_instance_type", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_type", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable
    for DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl
{
    type O = BlockAssignable<
        DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl {}

impl BuildDataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl {
    pub fn build(
        self,
    ) -> DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl {
        DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl {
            name: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesElRef {
        DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zones: Option<
        SetField<DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    host_instance_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_deployment_modes: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_engine_versions: Option<ListField<PrimField<String>>>,
}

impl DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        mut self,
        v: impl Into<
            SetField<DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesEl>,
        >,
    ) -> Self {
        self.availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `engine_type`.\n"]
    pub fn set_engine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engine_type = Some(v.into());
        self
    }

    #[doc = "Set the field `host_instance_type`.\n"]
    pub fn set_host_instance_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_instance_type = Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.storage_type = Some(v.into());
        self
    }

    #[doc = "Set the field `supported_deployment_modes`.\n"]
    pub fn set_supported_deployment_modes(
        mut self,
        v: impl Into<SetField<PrimField<String>>>,
    ) -> Self {
        self.supported_deployment_modes = Some(v.into());
        self
    }

    #[doc = "Set the field `supported_engine_versions`.\n"]
    pub fn set_supported_engine_versions(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.supported_engine_versions = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl {
    type O = BlockAssignable<DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl {}

impl BuildDataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl {
    pub fn build(self) -> DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl {
        DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsEl {
            availability_zones: core::default::Default::default(),
            engine_type: core::default::Default::default(),
            host_instance_type: core::default::Default::default(),
            storage_type: core::default::Default::default(),
            supported_deployment_modes: core::default::Default::default(),
            supported_engine_versions: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElRef {
        DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(
        &self,
    ) -> SetRef<DataMqBrokerInstanceTypeOfferingsBrokerInstanceOptionsElAvailabilityZonesElRef>
    {
        SetRef::new(
            self.shared().clone(),
            format!("{}.availability_zones", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_instance_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_deployment_modes` after provisioning.\n"]
    pub fn supported_deployment_modes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.supported_deployment_modes", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `supported_engine_versions` after provisioning.\n"]
    pub fn supported_engine_versions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.supported_engine_versions", self.base),
        )
    }
}
