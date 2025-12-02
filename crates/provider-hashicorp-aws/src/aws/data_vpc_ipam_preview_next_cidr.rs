use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataVpcIpamPreviewNextCidrData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disallowed_cidrs: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    ipam_pool_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    netmask_length: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataVpcIpamPreviewNextCidrTimeoutsEl>,
}
struct DataVpcIpamPreviewNextCidr_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataVpcIpamPreviewNextCidrData>,
}
#[derive(Clone)]
pub struct DataVpcIpamPreviewNextCidr(Rc<DataVpcIpamPreviewNextCidr_>);
impl DataVpcIpamPreviewNextCidr {
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
    #[doc = "Set the field `disallowed_cidrs`.\n"]
    pub fn set_disallowed_cidrs(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().disallowed_cidrs = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `netmask_length`.\n"]
    pub fn set_netmask_length(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().netmask_length = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataVpcIpamPreviewNextCidrTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disallowed_cidrs` after provisioning.\n"]
    pub fn disallowed_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.disallowed_cidrs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipam_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `netmask_length` after provisioning.\n"]
    pub fn netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.netmask_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPreviewNextCidrTimeoutsElRef {
        DataVpcIpamPreviewNextCidrTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataVpcIpamPreviewNextCidr {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataVpcIpamPreviewNextCidr {}
impl ToListMappable for DataVpcIpamPreviewNextCidr {
    type O = ListRef<DataVpcIpamPreviewNextCidrRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataVpcIpamPreviewNextCidr_ {
    fn extract_datasource_type(&self) -> String {
        "aws_vpc_ipam_preview_next_cidr".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataVpcIpamPreviewNextCidr {
    pub tf_id: String,
    #[doc = ""]
    pub ipam_pool_id: PrimField<String>,
}
impl BuildDataVpcIpamPreviewNextCidr {
    pub fn build(self, stack: &mut Stack) -> DataVpcIpamPreviewNextCidr {
        let out = DataVpcIpamPreviewNextCidr(Rc::new(DataVpcIpamPreviewNextCidr_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataVpcIpamPreviewNextCidrData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                disallowed_cidrs: core::default::Default::default(),
                id: core::default::Default::default(),
                ipam_pool_id: self.ipam_pool_id,
                netmask_length: core::default::Default::default(),
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataVpcIpamPreviewNextCidrRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcIpamPreviewNextCidrRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataVpcIpamPreviewNextCidrRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cidr", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `disallowed_cidrs` after provisioning.\n"]
    pub fn disallowed_cidrs(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.disallowed_cidrs", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipam_pool_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `netmask_length` after provisioning.\n"]
    pub fn netmask_length(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.netmask_length", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPreviewNextCidrTimeoutsElRef {
        DataVpcIpamPreviewNextCidrTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataVpcIpamPreviewNextCidrTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}
impl DataVpcIpamPreviewNextCidrTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}
impl ToListMappable for DataVpcIpamPreviewNextCidrTimeoutsEl {
    type O = BlockAssignable<DataVpcIpamPreviewNextCidrTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataVpcIpamPreviewNextCidrTimeoutsEl {}
impl BuildDataVpcIpamPreviewNextCidrTimeoutsEl {
    pub fn build(self) -> DataVpcIpamPreviewNextCidrTimeoutsEl {
        DataVpcIpamPreviewNextCidrTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}
pub struct DataVpcIpamPreviewNextCidrTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataVpcIpamPreviewNextCidrTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataVpcIpamPreviewNextCidrTimeoutsElRef {
        DataVpcIpamPreviewNextCidrTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataVpcIpamPreviewNextCidrTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
