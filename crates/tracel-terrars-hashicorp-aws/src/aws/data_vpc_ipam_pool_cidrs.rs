use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcIpamPoolCidrsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ipam_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataVpcIpamPoolCidrsFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataVpcIpamPoolCidrsTimeoutsEl>,
    dynamic: DataVpcIpamPoolCidrsDynamic,
}

struct DataVpcIpamPoolCidrs_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcIpamPoolCidrsData>,
}

#[derive(Clone)]
pub struct DataVpcIpamPoolCidrs(Rc<DataVpcIpamPoolCidrs_>);

impl DataVpcIpamPoolCidrs {
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
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataVpcIpamPoolCidrsFilterEl>>) -> Self {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataVpcIpamPoolCidrsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_cidrs` after provisioning.\n"]
    pub fn ipam_pool_cidrs(&self) -> SetRef<DataVpcIpamPoolCidrsIpamPoolCidrsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ipam_pool_cidrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPoolCidrsTimeoutsElRef {
        DataVpcIpamPoolCidrsTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DataVpcIpamPoolCidrs {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataVpcIpamPoolCidrs { }

impl ToListMappable for DataVpcIpamPoolCidrs {
    type O = ListRef<DataVpcIpamPoolCidrsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataVpcIpamPoolCidrs_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpc_ipam_pool_cidrs".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpamPoolCidrs {
    pub tf_id: String,
    #[doc = ""]
    pub ipam_pool_id: PrimField<String>,
}

impl BuildDataVpcIpamPoolCidrs {
    pub fn build(self, stack: &mut Stack) -> DataVpcIpamPoolCidrs {
        let out = DataVpcIpamPoolCidrs(Rc::new(DataVpcIpamPoolCidrs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcIpamPoolCidrsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                ipam_pool_id: self.ipam_pool_id,
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamPoolCidrsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamPoolCidrsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataVpcIpamPoolCidrsRef {
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

    #[doc = "Get a reference to the value of field `ipam_pool_cidrs` after provisioning.\n"]
    pub fn ipam_pool_cidrs(&self) -> SetRef<DataVpcIpamPoolCidrsIpamPoolCidrsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.ipam_pool_cidrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPoolCidrsTimeoutsElRef {
        DataVpcIpamPoolCidrsTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolCidrsIpamPoolCidrsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cidr: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
}

impl DataVpcIpamPoolCidrsIpamPoolCidrsEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cidr = Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.state = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcIpamPoolCidrsIpamPoolCidrsEl {
    type O = BlockAssignable<DataVpcIpamPoolCidrsIpamPoolCidrsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolCidrsIpamPoolCidrsEl {}

impl BuildDataVpcIpamPoolCidrsIpamPoolCidrsEl {
    pub fn build(self) -> DataVpcIpamPoolCidrsIpamPoolCidrsEl {
        DataVpcIpamPoolCidrsIpamPoolCidrsEl {
            cidr: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
        DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolCidrsFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataVpcIpamPoolCidrsFilterEl { }

impl ToListMappable for DataVpcIpamPoolCidrsFilterEl {
    type O = BlockAssignable<DataVpcIpamPoolCidrsFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolCidrsFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataVpcIpamPoolCidrsFilterEl {
    pub fn build(self) -> DataVpcIpamPoolCidrsFilterEl {
        DataVpcIpamPoolCidrsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcIpamPoolCidrsFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamPoolCidrsFilterElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamPoolCidrsFilterElRef {
        DataVpcIpamPoolCidrsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolCidrsFilterElRef {
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
pub struct DataVpcIpamPoolCidrsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataVpcIpamPoolCidrsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataVpcIpamPoolCidrsTimeoutsEl {
    type O = BlockAssignable<DataVpcIpamPoolCidrsTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolCidrsTimeoutsEl {}

impl BuildDataVpcIpamPoolCidrsTimeoutsEl {
    pub fn build(self) -> DataVpcIpamPoolCidrsTimeoutsEl {
        DataVpcIpamPoolCidrsTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataVpcIpamPoolCidrsTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataVpcIpamPoolCidrsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamPoolCidrsTimeoutsElRef {
        DataVpcIpamPoolCidrsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolCidrsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataVpcIpamPoolCidrsDynamic {
    filter: Option<DynamicBlock<DataVpcIpamPoolCidrsFilterEl>>,
}
