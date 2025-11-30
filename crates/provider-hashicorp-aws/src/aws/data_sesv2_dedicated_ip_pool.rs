use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSesv2DedicatedIpPoolData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    pool_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataSesv2DedicatedIpPool_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSesv2DedicatedIpPoolData>,
}

#[derive(Clone)]
pub struct DataSesv2DedicatedIpPool(Rc<DataSesv2DedicatedIpPool_>);

impl DataSesv2DedicatedIpPool {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_ips` after provisioning.\n"]
    pub fn dedicated_ips(&self) -> ListRef<DataSesv2DedicatedIpPoolDedicatedIpsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_ips", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\n"]
    pub fn scaling_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

impl Referable for DataSesv2DedicatedIpPool {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSesv2DedicatedIpPool {}

impl ToListMappable for DataSesv2DedicatedIpPool {
    type O = ListRef<DataSesv2DedicatedIpPoolRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSesv2DedicatedIpPool_ {
    fn extract_datasource_type(&self) -> String {
        "aws_sesv2_dedicated_ip_pool".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSesv2DedicatedIpPool {
    pub tf_id: String,
    #[doc = ""]
    pub pool_name: PrimField<String>,
}

impl BuildDataSesv2DedicatedIpPool {
    pub fn build(self, stack: &mut Stack) -> DataSesv2DedicatedIpPool {
        let out = DataSesv2DedicatedIpPool(Rc::new(DataSesv2DedicatedIpPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSesv2DedicatedIpPoolData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                pool_name: self.pool_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSesv2DedicatedIpPoolRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSesv2DedicatedIpPoolRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSesv2DedicatedIpPoolRef {
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

    #[doc = "Get a reference to the value of field `dedicated_ips` after provisioning.\n"]
    pub fn dedicated_ips(&self) -> ListRef<DataSesv2DedicatedIpPoolDedicatedIpsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.dedicated_ips", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pool_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\n"]
    pub fn scaling_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.scaling_mode", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataSesv2DedicatedIpPoolDedicatedIpsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warmup_percentage: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    warmup_status: Option<PrimField<String>>,
}

impl DataSesv2DedicatedIpPoolDedicatedIpsEl {
    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.ip = Some(v.into());
        self
    }

    #[doc = "Set the field `warmup_percentage`.\n"]
    pub fn set_warmup_percentage(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.warmup_percentage = Some(v.into());
        self
    }

    #[doc = "Set the field `warmup_status`.\n"]
    pub fn set_warmup_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.warmup_status = Some(v.into());
        self
    }
}

impl ToListMappable for DataSesv2DedicatedIpPoolDedicatedIpsEl {
    type O = BlockAssignable<DataSesv2DedicatedIpPoolDedicatedIpsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2DedicatedIpPoolDedicatedIpsEl {}

impl BuildDataSesv2DedicatedIpPoolDedicatedIpsEl {
    pub fn build(self) -> DataSesv2DedicatedIpPoolDedicatedIpsEl {
        DataSesv2DedicatedIpPoolDedicatedIpsEl {
            ip: core::default::Default::default(),
            warmup_percentage: core::default::Default::default(),
            warmup_status: core::default::Default::default(),
        }
    }
}

pub struct DataSesv2DedicatedIpPoolDedicatedIpsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSesv2DedicatedIpPoolDedicatedIpsElRef {
    fn new(shared: StackShared, base: String) -> DataSesv2DedicatedIpPoolDedicatedIpsElRef {
        DataSesv2DedicatedIpPoolDedicatedIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2DedicatedIpPoolDedicatedIpsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `warmup_percentage` after provisioning.\n"]
    pub fn warmup_percentage(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warmup_percentage", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `warmup_status` after provisioning.\n"]
    pub fn warmup_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.warmup_status", self.base),
        )
    }
}
