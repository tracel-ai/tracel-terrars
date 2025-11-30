use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataApiGatewayApiKeysData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_values: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataApiGatewayApiKeys_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataApiGatewayApiKeysData>,
}

#[derive(Clone)]
pub struct DataApiGatewayApiKeys(Rc<DataApiGatewayApiKeys_>);

impl DataApiGatewayApiKeys {
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

    #[doc = "Set the field `customer_id`.\n"]
    pub fn set_customer_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().customer_id = Some(v.into());
        self
    }

    #[doc = "Set the field `include_values`.\n"]
    pub fn set_include_values(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_values = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `customer_id` after provisioning.\n"]
    pub fn customer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_values` after provisioning.\n"]
    pub fn include_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_values", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<DataApiGatewayApiKeysItemsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.items", self.extract_ref()),
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

impl Referable for DataApiGatewayApiKeys {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataApiGatewayApiKeys {}

impl ToListMappable for DataApiGatewayApiKeys {
    type O = ListRef<DataApiGatewayApiKeysRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataApiGatewayApiKeys_ {
    fn extract_datasource_type(&self) -> String {
        "aws_api_gateway_api_keys".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApiGatewayApiKeys {
    pub tf_id: String,
}

impl BuildDataApiGatewayApiKeys {
    pub fn build(self, stack: &mut Stack) -> DataApiGatewayApiKeys {
        let out = DataApiGatewayApiKeys(Rc::new(DataApiGatewayApiKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataApiGatewayApiKeysData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                customer_id: core::default::Default::default(),
                include_values: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApiGatewayApiKeysRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataApiGatewayApiKeysRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataApiGatewayApiKeysRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `customer_id` after provisioning.\n"]
    pub fn customer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.customer_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_values` after provisioning.\n"]
    pub fn include_values(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_values", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> ListRef<DataApiGatewayApiKeysItemsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.items", self.extract_ref()),
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
pub struct DataApiGatewayApiKeysItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_updated_date: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage_keys: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DataApiGatewayApiKeysItemsEl {
    #[doc = "Set the field `created_date`.\n"]
    pub fn set_created_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.created_date = Some(v.into());
        self
    }

    #[doc = "Set the field `customer_id`.\n"]
    pub fn set_customer_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.customer_id = Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.id = Some(v.into());
        self
    }

    #[doc = "Set the field `last_updated_date`.\n"]
    pub fn set_last_updated_date(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_updated_date = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }

    #[doc = "Set the field `stage_keys`.\n"]
    pub fn set_stage_keys(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.stage_keys = Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.tags = Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DataApiGatewayApiKeysItemsEl {
    type O = BlockAssignable<DataApiGatewayApiKeysItemsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataApiGatewayApiKeysItemsEl {}

impl BuildDataApiGatewayApiKeysItemsEl {
    pub fn build(self) -> DataApiGatewayApiKeysItemsEl {
        DataApiGatewayApiKeysItemsEl {
            created_date: core::default::Default::default(),
            customer_id: core::default::Default::default(),
            description: core::default::Default::default(),
            enabled: core::default::Default::default(),
            id: core::default::Default::default(),
            last_updated_date: core::default::Default::default(),
            name: core::default::Default::default(),
            stage_keys: core::default::Default::default(),
            tags: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DataApiGatewayApiKeysItemsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataApiGatewayApiKeysItemsElRef {
    fn new(shared: StackShared, base: String) -> DataApiGatewayApiKeysItemsElRef {
        DataApiGatewayApiKeysItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataApiGatewayApiKeysItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_id` after provisioning.\n"]
    pub fn customer_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.customer_id", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_updated_date", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `stage_keys` after provisioning.\n"]
    pub fn stage_keys(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.stage_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}
