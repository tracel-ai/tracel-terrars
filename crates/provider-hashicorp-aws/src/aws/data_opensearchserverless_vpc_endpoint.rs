use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataOpensearchserverlessVpcEndpointData {
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
    vpc_endpoint_id: PrimField<String>,
}
struct DataOpensearchserverlessVpcEndpoint_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOpensearchserverlessVpcEndpointData>,
}
#[derive(Clone)]
pub struct DataOpensearchserverlessVpcEndpoint(Rc<DataOpensearchserverlessVpcEndpoint_>);
impl DataOpensearchserverlessVpcEndpoint {
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
    #[doc = "Get a reference to the value of field `created_date` after provisioning.\nThe date the endpoint was created."]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the endpoint."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\nThe IDs of the security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint."]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\nThe IDs of the subnets from which you access OpenSearch Serverless."]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\nThe unique identifier of the endpoint."]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\nThe ID of the VPC from which you access OpenSearch Serverless."]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
}
impl Referable for DataOpensearchserverlessVpcEndpoint {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataOpensearchserverlessVpcEndpoint {}
impl ToListMappable for DataOpensearchserverlessVpcEndpoint {
    type O = ListRef<DataOpensearchserverlessVpcEndpointRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataOpensearchserverlessVpcEndpoint_ {
    fn extract_datasource_type(&self) -> String {
        "aws_opensearchserverless_vpc_endpoint".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataOpensearchserverlessVpcEndpoint {
    pub tf_id: String,
    #[doc = "The unique identifier of the endpoint."]
    pub vpc_endpoint_id: PrimField<String>,
}
impl BuildDataOpensearchserverlessVpcEndpoint {
    pub fn build(self, stack: &mut Stack) -> DataOpensearchserverlessVpcEndpoint {
        let out =
            DataOpensearchserverlessVpcEndpoint(Rc::new(DataOpensearchserverlessVpcEndpoint_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataOpensearchserverlessVpcEndpointData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    vpc_endpoint_id: self.vpc_endpoint_id,
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataOpensearchserverlessVpcEndpointRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataOpensearchserverlessVpcEndpointRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataOpensearchserverlessVpcEndpointRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `created_date` after provisioning.\nThe date the endpoint was created."]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_date", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe name of the endpoint."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\nThe IDs of the security groups that define the ports, protocols, and sources for inbound traffic that you are authorizing into your endpoint."]
    pub fn security_group_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_group_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\nThe IDs of the subnets from which you access OpenSearch Serverless."]
    pub fn subnet_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.subnet_ids", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\nThe unique identifier of the endpoint."]
    pub fn vpc_endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_endpoint_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\nThe ID of the VPC from which you access OpenSearch Serverless."]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.vpc_id", self.extract_ref()),
        )
    }
}
