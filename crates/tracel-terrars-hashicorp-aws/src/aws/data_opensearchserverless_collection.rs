use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOpensearchserverlessCollectionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataOpensearchserverlessCollection_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataOpensearchserverlessCollectionData>,
}

#[derive(Clone)]
pub struct DataOpensearchserverlessCollection(Rc<DataOpensearchserverlessCollection_>);

impl DataOpensearchserverlessCollection {
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

    #[doc = "Set the field `id`.\nID of the collection."]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\nName of the collection."]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `collection_endpoint` after provisioning.\nCollection-specific endpoint used to submit index, search, and data upload requests to an OpenSearch Serverless collection."]
    pub fn collection_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nDate the Collection was created."]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dashboard_endpoint` after provisioning.\nCollection-specific endpoint used to access OpenSearch Dashboards."]
    pub fn dashboard_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the collection."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_code` after provisioning.\nA failure code associated with the collection."]
    pub fn failure_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_message` after provisioning.\nA failure reason associated with the collection."]
    pub fn failure_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the collection."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_arn` after provisioning.\nThe ARN of the Amazon Web Services KMS key used to encrypt the collection."]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_date` after provisioning.\nDate the Collection was last modified."]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the collection."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_replicas` after provisioning.\nIndicates whether standby replicas should be used for a collection."]
    pub fn standby_replicas(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standby_replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of collection."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Referable for DataOpensearchserverlessCollection {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataOpensearchserverlessCollection { }

impl ToListMappable for DataOpensearchserverlessCollection {
    type O = ListRef<DataOpensearchserverlessCollectionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataOpensearchserverlessCollection_ {
    fn extract_datasource_type(&self) -> String {
        "aws_opensearchserverless_collection".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOpensearchserverlessCollection {
    pub tf_id: String,
}

impl BuildDataOpensearchserverlessCollection {
    pub fn build(self, stack: &mut Stack) -> DataOpensearchserverlessCollection {
        let out = DataOpensearchserverlessCollection(Rc::new(DataOpensearchserverlessCollection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataOpensearchserverlessCollectionData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOpensearchserverlessCollectionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataOpensearchserverlessCollectionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataOpensearchserverlessCollectionRef {
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

    #[doc =
        "Get a reference to the value of field `collection_endpoint` after provisioning.\nCollection-specific endpoint used to submit index, search, and data upload requests to an OpenSearch Serverless collection."]
    pub fn collection_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.collection_endpoint", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `created_date` after provisioning.\nDate the Collection was created."]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `dashboard_endpoint` after provisioning.\nCollection-specific endpoint used to access OpenSearch Dashboards."]
    pub fn dashboard_endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.dashboard_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the collection."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_code` after provisioning.\nA failure code associated with the collection."]
    pub fn failure_code(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_code", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `failure_message` after provisioning.\nA failure reason associated with the collection."]
    pub fn failure_message(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.failure_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nID of the collection."]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `kms_key_arn` after provisioning.\nThe ARN of the Amazon Web Services KMS key used to encrypt the collection."]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `last_modified_date` after provisioning.\nDate the Collection was last modified."]
    pub fn last_modified_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_modified_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the collection."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `standby_replicas` after provisioning.\nIndicates whether standby replicas should be used for a collection."]
    pub fn standby_replicas(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.standby_replicas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\nType of collection."]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}
