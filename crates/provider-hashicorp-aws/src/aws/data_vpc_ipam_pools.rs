use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcIpamPoolsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataVpcIpamPoolsFilterEl>>,
    dynamic: DataVpcIpamPoolsDynamic,
}

struct DataVpcIpamPools_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcIpamPoolsData>,
}

#[derive(Clone)]
pub struct DataVpcIpamPools(Rc<DataVpcIpamPools_>);

impl DataVpcIpamPools {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataVpcIpamPoolsFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> SetRef<DataVpcIpamPoolsIpamPoolsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ipam_pools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataVpcIpamPools {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataVpcIpamPools { }

impl ToListMappable for DataVpcIpamPools {
    type O = ListRef<DataVpcIpamPoolsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVpcIpamPools_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpc_ipam_pools".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpamPools {
    pub tf_id: String,
}

impl BuildDataVpcIpamPools {
    pub fn build(self, stack: &mut Stack) -> DataVpcIpamPools {
        let out = DataVpcIpamPools(Rc::new(DataVpcIpamPools_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcIpamPoolsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamPoolsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamPoolsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataVpcIpamPoolsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pools` after provisioning.\n"]
    pub fn ipam_pools(&self) -> SetRef<DataVpcIpamPoolsIpamPoolsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ipam_pools", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolsIpamPoolsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    address_family: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_default_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_max_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_min_netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allocation_resource_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_import: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_service: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_scope_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_scope_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pool_depth: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publicly_advertisable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_ipam_pool_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

impl DataVpcIpamPoolsIpamPoolsEl {
    #[doc = "Set the field `address_family`.\n"]
    pub fn set_address_family(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.address_family = Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_default_netmask_length`.\n"]
    pub fn set_allocation_default_netmask_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.allocation_default_netmask_length = Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_max_netmask_length`.\n"]
    pub fn set_allocation_max_netmask_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.allocation_max_netmask_length = Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_min_netmask_length`.\n"]
    pub fn set_allocation_min_netmask_length(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.allocation_min_netmask_length = Some(v.into());
        self
    }

    #[doc = "Set the field `allocation_resource_tags`.\n"]
    pub fn set_allocation_resource_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.allocation_resource_tags = Some(v.into());
        self
    }

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `auto_import`.\n"]
    pub fn set_auto_import(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.auto_import = Some(v.into());
        self
    }

    #[doc = "Set the field `aws_service`.\n"]
    pub fn set_aws_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.aws_service = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_scope_id`.\n"]
    pub fn set_ipam_scope_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipam_scope_id = Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_scope_type`.\n"]
    pub fn set_ipam_scope_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipam_scope_type = Some(v.into());
        self
    }

    #[doc = "Set the field `locale`.\n"]
    pub fn set_locale(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.locale = Some(v.into());
        self
    }

    #[doc = "Set the field `pool_depth`.\n"]
    pub fn set_pool_depth(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.pool_depth = Some(v.into());
        self
    }

    #[doc = "Set the field `publicly_advertisable`.\n"]
    pub fn set_publicly_advertisable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.publicly_advertisable = Some(v.into());
        self
    }

    #[doc = "Set the field `source_ipam_pool_id`.\n"]
    pub fn set_source_ipam_pool_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.source_ipam_pool_id = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcIpamPoolsIpamPoolsEl {
    type O = BlockAssignable<DataVpcIpamPoolsIpamPoolsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolsIpamPoolsEl {}

impl BuildDataVpcIpamPoolsIpamPoolsEl {
    pub fn build(self) -> DataVpcIpamPoolsIpamPoolsEl {
        DataVpcIpamPoolsIpamPoolsEl {
            address_family: core::default::Default::default(),
            allocation_default_netmask_length: core::default::Default::default(),
            allocation_max_netmask_length: core::default::Default::default(),
            allocation_min_netmask_length: core::default::Default::default(),
            allocation_resource_tags: core::default::Default::default(),
            arn: core::default::Default::default(),
            auto_import: core::default::Default::default(),
            aws_service: core::default::Default::default(),
            description: core::default::Default::default(),
            id: core::default::Default::default(),
            ipam_scope_id: core::default::Default::default(),
            ipam_scope_type: core::default::Default::default(),
            locale: core::default::Default::default(),
            pool_depth: core::default::Default::default(),
            publicly_advertisable: core::default::Default::default(),
            source_ipam_pool_id: core::default::Default::default(),
            state: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataVpcIpamPoolsIpamPoolsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamPoolsIpamPoolsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamPoolsIpamPoolsElRef {
        DataVpcIpamPoolsIpamPoolsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolsIpamPoolsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_default_netmask_length` after provisioning.\n"]
    pub fn allocation_default_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_default_netmask_length", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_max_netmask_length` after provisioning.\n"]
    pub fn allocation_max_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_max_netmask_length", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_min_netmask_length` after provisioning.\n"]
    pub fn allocation_min_netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.allocation_min_netmask_length", self.base))
    }

    #[doc = "Get a reference to the value of field `allocation_resource_tags` after provisioning.\n"]
    pub fn allocation_resource_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.allocation_resource_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_import` after provisioning.\n"]
    pub fn auto_import(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_import", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_service` after provisioning.\n"]
    pub fn aws_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.aws_service", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_id` after provisioning.\n"]
    pub fn ipam_scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipam_scope_type` after provisioning.\n"]
    pub fn ipam_scope_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_scope_type", self.base))
    }

    #[doc = "Get a reference to the value of field `locale` after provisioning.\n"]
    pub fn locale(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.locale", self.base))
    }

    #[doc = "Get a reference to the value of field `pool_depth` after provisioning.\n"]
    pub fn pool_depth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.pool_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `publicly_advertisable` after provisioning.\n"]
    pub fn publicly_advertisable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.publicly_advertisable", self.base))
    }

    #[doc = "Get a reference to the value of field `source_ipam_pool_id` after provisioning.\n"]
    pub fn source_ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_ipam_pool_id", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolsFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataVpcIpamPoolsFilterEl { }

impl ToListMappable for DataVpcIpamPoolsFilterEl {
    type O = BlockAssignable<DataVpcIpamPoolsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolsFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataVpcIpamPoolsFilterEl {
    pub fn build(self) -> DataVpcIpamPoolsFilterEl {
        DataVpcIpamPoolsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcIpamPoolsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamPoolsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamPoolsFilterElRef {
        DataVpcIpamPoolsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolsFilterElRef {
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

#[derive(Serialize, Default)]
struct DataVpcIpamPoolsDynamic {
    filter: Option<DynamicBlock<DataVpcIpamPoolsFilterEl>>,
}
