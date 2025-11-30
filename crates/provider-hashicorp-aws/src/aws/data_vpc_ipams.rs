use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataVpcIpamsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_ids: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataVpcIpamsFilterEl>>,
    dynamic: DataVpcIpamsDynamic,
}

struct DataVpcIpams_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcIpamsData>,
}

#[derive(Clone)]
pub struct DataVpcIpams(Rc<DataVpcIpams_>);

impl DataVpcIpams {
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

    #[doc = "Set the field `ipam_ids`.\n"]
    pub fn set_ipam_ids(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().ipam_ids = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataVpcIpamsFilterEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `ipam_ids` after provisioning.\n"]
    pub fn ipam_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipam_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipams` after provisioning.\n"]
    pub fn ipams(&self) -> ListRef<DataVpcIpamsIpamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipams", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

impl Referable for DataVpcIpams {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataVpcIpams {}

impl ToListMappable for DataVpcIpams {
    type O = ListRef<DataVpcIpamsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVpcIpams_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpc_ipams".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpams {
    pub tf_id: String,
}

impl BuildDataVpcIpams {
    pub fn build(self, stack: &mut Stack) -> DataVpcIpams {
        let out = DataVpcIpams(Rc::new(DataVpcIpams_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcIpamsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                ipam_ids: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataVpcIpamsRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `ipam_ids` after provisioning.\n"]
    pub fn ipam_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipam_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipams` after provisioning.\n"]
    pub fn ipams(&self) -> ListRef<DataVpcIpamsIpamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ipams", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamsIpamsElOperatingRegionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    region_name: Option<PrimField<String>>,
}

impl DataVpcIpamsIpamsElOperatingRegionsEl {
    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region_name = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcIpamsIpamsElOperatingRegionsEl {
    type O = BlockAssignable<DataVpcIpamsIpamsElOperatingRegionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamsIpamsElOperatingRegionsEl {}

impl BuildDataVpcIpamsIpamsElOperatingRegionsEl {
    pub fn build(self) -> DataVpcIpamsIpamsElOperatingRegionsEl {
        DataVpcIpamsIpamsElOperatingRegionsEl {
            region_name: core::default::Default::default(),
        }
    }
}

pub struct DataVpcIpamsIpamsElOperatingRegionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamsIpamsElOperatingRegionsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamsIpamsElOperatingRegionsElRef {
        DataVpcIpamsIpamsElOperatingRegionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamsIpamsElOperatingRegionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamsIpamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_discovery_association_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_resource_discovery_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_private_gua: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ipam_region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metered_account: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_regions: Option<ListField<DataVpcIpamsIpamsElOperatingRegionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    private_default_scope_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_default_scope_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_discovery_association_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_message: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<PrimField<String>>,
}

impl DataVpcIpamsIpamsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `default_resource_discovery_association_id`.\n"]
    pub fn set_default_resource_discovery_association_id(
        mut self,
        v: impl Into<PrimField<String>>,
    ) -> Self {
        self.default_resource_discovery_association_id = Some(v.into());
        self
    }

    #[doc = "Set the field `default_resource_discovery_id`.\n"]
    pub fn set_default_resource_discovery_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_resource_discovery_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_private_gua`.\n"]
    pub fn set_enable_private_gua(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_private_gua = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `ipam_region`.\n"]
    pub fn set_ipam_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ipam_region = Some(v.into());
        self
    }

    #[doc = "Set the field `metered_account`.\n"]
    pub fn set_metered_account(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.metered_account = Some(v.into());
        self
    }

    #[doc = "Set the field `operating_regions`.\n"]
    pub fn set_operating_regions(
        mut self,
        v: impl Into<ListField<DataVpcIpamsIpamsElOperatingRegionsEl>>,
    ) -> Self {
        self.operating_regions = Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\n"]
    pub fn set_owner_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owner_id = Some(v.into());
        self
    }

    #[doc = "Set the field `private_default_scope_id`.\n"]
    pub fn set_private_default_scope_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.private_default_scope_id = Some(v.into());
        self
    }

    #[doc = "Set the field `public_default_scope_id`.\n"]
    pub fn set_public_default_scope_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.public_default_scope_id = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_discovery_association_count`.\n"]
    pub fn set_resource_discovery_association_count(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.resource_discovery_association_count = Some(v.into());
        self
    }

    #[doc = "Set the field `scope_count`.\n"]
    pub fn set_scope_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.scope_count = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }

    #[doc = "Set the field `state_message`.\n"]
    pub fn set_state_message(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state_message = Some(v.into());
        self
    }

    #[doc = "Set the field `tier`.\n"]
    pub fn set_tier(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.tier = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcIpamsIpamsEl {
    type O = BlockAssignable<DataVpcIpamsIpamsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamsIpamsEl {}

impl BuildDataVpcIpamsIpamsEl {
    pub fn build(self) -> DataVpcIpamsIpamsEl {
        DataVpcIpamsIpamsEl {
            arn: core::default::Default::default(),
            default_resource_discovery_association_id: core::default::Default::default(),
            default_resource_discovery_id: core::default::Default::default(),
            description: core::default::Default::default(),
            enable_private_gua: core::default::Default::default(),
            id: core::default::Default::default(),
            ipam_region: core::default::Default::default(),
            metered_account: core::default::Default::default(),
            operating_regions: core::default::Default::default(),
            owner_id: core::default::Default::default(),
            private_default_scope_id: core::default::Default::default(),
            public_default_scope_id: core::default::Default::default(),
            resource_discovery_association_count: core::default::Default::default(),
            scope_count: core::default::Default::default(),
            state: core::default::Default::default(),
            state_message: core::default::Default::default(),
            tier: core::default::Default::default(),
        }
    }
}

pub struct DataVpcIpamsIpamsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamsIpamsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamsIpamsElRef {
        DataVpcIpamsIpamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamsIpamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `default_resource_discovery_association_id` after provisioning.\n"]
    pub fn default_resource_discovery_association_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_resource_discovery_association_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `default_resource_discovery_id` after provisioning.\n"]
    pub fn default_resource_discovery_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_resource_discovery_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_private_gua` after provisioning.\n"]
    pub fn enable_private_gua(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_private_gua", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `ipam_region` after provisioning.\n"]
    pub fn ipam_region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_region", self.base))
    }

    #[doc = "Get a reference to the value of field `metered_account` after provisioning.\n"]
    pub fn metered_account(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.metered_account", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `operating_regions` after provisioning.\n"]
    pub fn operating_regions(&self) -> ListRef<DataVpcIpamsIpamsElOperatingRegionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.operating_regions", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }

    #[doc = "Get a reference to the value of field `private_default_scope_id` after provisioning.\n"]
    pub fn private_default_scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_default_scope_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `public_default_scope_id` after provisioning.\n"]
    pub fn public_default_scope_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.public_default_scope_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `resource_discovery_association_count` after provisioning.\n"]
    pub fn resource_discovery_association_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_discovery_association_count", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `scope_count` after provisioning.\n"]
    pub fn scope_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope_count", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `state_message` after provisioning.\n"]
    pub fn state_message(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state_message", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `tier` after provisioning.\n"]
    pub fn tier(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.tier", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamsFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataVpcIpamsFilterEl {}

impl ToListMappable for DataVpcIpamsFilterEl {
    type O = BlockAssignable<DataVpcIpamsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamsFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataVpcIpamsFilterEl {
    pub fn build(self) -> DataVpcIpamsFilterEl {
        DataVpcIpamsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcIpamsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamsFilterElRef {
        DataVpcIpamsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamsFilterElRef {
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
struct DataVpcIpamsDynamic {
    filter: Option<DynamicBlock<DataVpcIpamsFilterEl>>,
}
