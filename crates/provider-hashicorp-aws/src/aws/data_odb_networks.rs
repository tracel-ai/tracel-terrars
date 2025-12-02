use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOdbNetworksData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
struct DataOdbNetworks_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOdbNetworksData>,
}
#[derive(Clone)]
pub struct DataOdbNetworks(Rc<DataOdbNetworks_>);
impl DataOdbNetworks {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `odb_networks` after provisioning.\nList of odb networks returns basic information about odb networks."]
    pub fn odb_networks(&self) -> ListRef<DataOdbNetworksOdbNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.odb_networks", self.extract_ref()),
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
impl Referable for DataOdbNetworks {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOdbNetworks {}
impl ToListMappable for DataOdbNetworks {
    type O = ListRef<DataOdbNetworksRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOdbNetworks_ {
    fn extract_datasource_type(&self) -> String {
        "aws_odb_networks".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOdbNetworks {
    pub tf_id: String,
}
impl BuildDataOdbNetworks {
    pub fn build(self, stack: &mut Stack) -> DataOdbNetworks {
        let out = DataOdbNetworks(Rc::new(DataOdbNetworks_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOdbNetworksData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOdbNetworksRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbNetworksRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOdbNetworksRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `odb_networks` after provisioning.\nList of odb networks returns basic information about odb networks."]
    pub fn odb_networks(&self) -> ListRef<DataOdbNetworksOdbNetworksElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.odb_networks", self.extract_ref()),
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
pub struct DataOdbNetworksOdbNetworksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_network_anchor_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_vcn_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    oci_vcn_url: Option<PrimField<String>>,
}
impl DataOdbNetworksOdbNetworksEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }
    #[doc = "Set the field `oci_network_anchor_id`.\n"]
    pub fn set_oci_network_anchor_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_network_anchor_id = Some(v.into());
        self
    }
    #[doc = "Set the field `oci_vcn_id`.\n"]
    pub fn set_oci_vcn_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_vcn_id = Some(v.into());
        self
    }
    #[doc = "Set the field `oci_vcn_url`.\n"]
    pub fn set_oci_vcn_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.oci_vcn_url = Some(v.into());
        self
    }
}
impl ToListMappable for DataOdbNetworksOdbNetworksEl {
    type O = BlockAssignable<DataOdbNetworksOdbNetworksEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataOdbNetworksOdbNetworksEl {}
impl BuildDataOdbNetworksOdbNetworksEl {
    pub fn build(self) -> DataOdbNetworksOdbNetworksEl {
        DataOdbNetworksOdbNetworksEl {
            arn: core::default::Default::default(),
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            oci_network_anchor_id: core::default::Default::default(),
            oci_vcn_id: core::default::Default::default(),
            oci_vcn_url: core::default::Default::default(),
        }
    }
}
pub struct DataOdbNetworksOdbNetworksElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOdbNetworksOdbNetworksElRef {
    fn new(shared: StackShared, base: String) -> DataOdbNetworksOdbNetworksElRef {
        DataOdbNetworksOdbNetworksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataOdbNetworksOdbNetworksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `oci_network_anchor_id` after provisioning.\n"]
    pub fn oci_network_anchor_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.oci_network_anchor_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `oci_vcn_id` after provisioning.\n"]
    pub fn oci_vcn_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_id", self.base))
    }
    #[doc = "Get a reference to the value of field `oci_vcn_url` after provisioning.\n"]
    pub fn oci_vcn_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.oci_vcn_url", self.base))
    }
}
