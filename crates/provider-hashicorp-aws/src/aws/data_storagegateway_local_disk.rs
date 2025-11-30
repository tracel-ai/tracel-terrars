use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataStoragegatewayLocalDiskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_node: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk_path: Option<PrimField<String>>,
    gateway_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataStoragegatewayLocalDisk_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataStoragegatewayLocalDiskData>,
}

#[derive(Clone)]
pub struct DataStoragegatewayLocalDisk(Rc<DataStoragegatewayLocalDisk_>);

impl DataStoragegatewayLocalDisk {
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

    #[doc = "Set the field `disk_node`.\n"]
    pub fn set_disk_node(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().disk_node = Some(v.into());
        self
    }

    #[doc = "Set the field `disk_path`.\n"]
    pub fn set_disk_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().disk_path = Some(v.into());
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

    #[doc = "Get a reference to the value of field `disk_id` after provisioning.\n"]
    pub fn disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_node` after provisioning.\n"]
    pub fn disk_node(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_node", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_path` after provisioning.\n"]
    pub fn disk_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
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
}

impl Referable for DataStoragegatewayLocalDisk {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataStoragegatewayLocalDisk {}

impl ToListMappable for DataStoragegatewayLocalDisk {
    type O = ListRef<DataStoragegatewayLocalDiskRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataStoragegatewayLocalDisk_ {
    fn extract_datasource_type(&self) -> String {
        "aws_storagegateway_local_disk".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStoragegatewayLocalDisk {
    pub tf_id: String,
    #[doc = ""]
    pub gateway_arn: PrimField<String>,
}

impl BuildDataStoragegatewayLocalDisk {
    pub fn build(self, stack: &mut Stack) -> DataStoragegatewayLocalDisk {
        let out = DataStoragegatewayLocalDisk(Rc::new(DataStoragegatewayLocalDisk_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataStoragegatewayLocalDiskData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                disk_node: core::default::Default::default(),
                disk_path: core::default::Default::default(),
                gateway_arn: self.gateway_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStoragegatewayLocalDiskRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataStoragegatewayLocalDiskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataStoragegatewayLocalDiskRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `disk_id` after provisioning.\n"]
    pub fn disk_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_node` after provisioning.\n"]
    pub fn disk_node(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_node", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `disk_path` after provisioning.\n"]
    pub fn disk_path(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.disk_path", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.gateway_arn", self.extract_ref()),
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
}
