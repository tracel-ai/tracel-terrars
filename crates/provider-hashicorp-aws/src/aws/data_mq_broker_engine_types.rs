use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMqBrokerEngineTypesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataMqBrokerEngineTypes_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataMqBrokerEngineTypesData>,
}

#[derive(Clone)]
pub struct DataMqBrokerEngineTypes(Rc<DataMqBrokerEngineTypes_>);

impl DataMqBrokerEngineTypes {
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

    #[doc = "Get a reference to the value of field `broker_engine_types` after provisioning.\n"]
    pub fn broker_engine_types(&self) -> ListRef<DataMqBrokerEngineTypesBrokerEngineTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.broker_engine_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl Referable for DataMqBrokerEngineTypes {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataMqBrokerEngineTypes { }

impl ToListMappable for DataMqBrokerEngineTypes {
    type O = ListRef<DataMqBrokerEngineTypesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataMqBrokerEngineTypes_ {
    fn extract_datasource_type(&self) -> String {
        "aws_mq_broker_engine_types".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMqBrokerEngineTypes {
    pub tf_id: String,
}

impl BuildDataMqBrokerEngineTypes {
    pub fn build(self, stack: &mut Stack) -> DataMqBrokerEngineTypes {
        let out = DataMqBrokerEngineTypes(Rc::new(DataMqBrokerEngineTypes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataMqBrokerEngineTypesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                engine_type: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMqBrokerEngineTypesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerEngineTypesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataMqBrokerEngineTypesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `broker_engine_types` after provisioning.\n"]
    pub fn broker_engine_types(&self) -> ListRef<DataMqBrokerEngineTypesBrokerEngineTypesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.broker_engine_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl {
    type O = BlockAssignable<DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl {}

impl BuildDataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl {
    pub fn build(self) -> DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl {
        DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl { name: core::default::Default::default() }
    }
}

pub struct DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsElRef {
        DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerEngineTypesBrokerEngineTypesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    engine_versions: Option<ListField<DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl>>,
}

impl DataMqBrokerEngineTypesBrokerEngineTypesEl {
    #[doc = "Set the field `engine_type`.\n"]
    pub fn set_engine_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.engine_type = Some(v.into());
        self
    }

    #[doc = "Set the field `engine_versions`.\n"]
    pub fn set_engine_versions(
        mut self,
        v: impl Into<ListField<DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsEl>>,
    ) -> Self {
        self.engine_versions = Some(v.into());
        self
    }
}

impl ToListMappable for DataMqBrokerEngineTypesBrokerEngineTypesEl {
    type O = BlockAssignable<DataMqBrokerEngineTypesBrokerEngineTypesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerEngineTypesBrokerEngineTypesEl {}

impl BuildDataMqBrokerEngineTypesBrokerEngineTypesEl {
    pub fn build(self) -> DataMqBrokerEngineTypesBrokerEngineTypesEl {
        DataMqBrokerEngineTypesBrokerEngineTypesEl {
            engine_type: core::default::Default::default(),
            engine_versions: core::default::Default::default(),
        }
    }
}

pub struct DataMqBrokerEngineTypesBrokerEngineTypesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataMqBrokerEngineTypesBrokerEngineTypesElRef {
    fn new(shared: StackShared, base: String) -> DataMqBrokerEngineTypesBrokerEngineTypesElRef {
        DataMqBrokerEngineTypesBrokerEngineTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerEngineTypesBrokerEngineTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_versions` after provisioning.\n"]
    pub fn engine_versions(&self) -> ListRef<DataMqBrokerEngineTypesBrokerEngineTypesElEngineVersionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.engine_versions", self.base))
    }
}
