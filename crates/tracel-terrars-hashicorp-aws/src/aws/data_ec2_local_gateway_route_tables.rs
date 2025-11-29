use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2LocalGatewayRouteTablesData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataEc2LocalGatewayRouteTablesFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2LocalGatewayRouteTablesTimeoutsEl>,
    dynamic: DataEc2LocalGatewayRouteTablesDynamic,
}

struct DataEc2LocalGatewayRouteTables_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2LocalGatewayRouteTablesData>,
}

#[derive(Clone)]
pub struct DataEc2LocalGatewayRouteTables(Rc<DataEc2LocalGatewayRouteTables_>);

impl DataEc2LocalGatewayRouteTables {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataEc2LocalGatewayRouteTablesFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataEc2LocalGatewayRouteTablesTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayRouteTablesTimeoutsElRef {
        DataEc2LocalGatewayRouteTablesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataEc2LocalGatewayRouteTables {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2LocalGatewayRouteTables { }

impl ToListMappable for DataEc2LocalGatewayRouteTables {
    type O = ListRef<DataEc2LocalGatewayRouteTablesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2LocalGatewayRouteTables_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_local_gateway_route_tables".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2LocalGatewayRouteTables {
    pub tf_id: String,
}

impl BuildDataEc2LocalGatewayRouteTables {
    pub fn build(self, stack: &mut Stack) -> DataEc2LocalGatewayRouteTables {
        let out = DataEc2LocalGatewayRouteTables(Rc::new(DataEc2LocalGatewayRouteTables_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2LocalGatewayRouteTablesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2LocalGatewayRouteTablesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayRouteTablesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataEc2LocalGatewayRouteTablesRef {
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

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayRouteTablesTimeoutsElRef {
        DataEc2LocalGatewayRouteTablesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2LocalGatewayRouteTablesFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataEc2LocalGatewayRouteTablesFilterEl { }

impl ToListMappable for DataEc2LocalGatewayRouteTablesFilterEl {
    type O = BlockAssignable<DataEc2LocalGatewayRouteTablesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayRouteTablesFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataEc2LocalGatewayRouteTablesFilterEl {
    pub fn build(self) -> DataEc2LocalGatewayRouteTablesFilterEl {
        DataEc2LocalGatewayRouteTablesFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2LocalGatewayRouteTablesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayRouteTablesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataEc2LocalGatewayRouteTablesFilterElRef {
        DataEc2LocalGatewayRouteTablesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayRouteTablesFilterElRef {
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
pub struct DataEc2LocalGatewayRouteTablesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2LocalGatewayRouteTablesTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2LocalGatewayRouteTablesTimeoutsEl {
    type O = BlockAssignable<DataEc2LocalGatewayRouteTablesTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayRouteTablesTimeoutsEl {}

impl BuildDataEc2LocalGatewayRouteTablesTimeoutsEl {
    pub fn build(self) -> DataEc2LocalGatewayRouteTablesTimeoutsEl {
        DataEc2LocalGatewayRouteTablesTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2LocalGatewayRouteTablesTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2LocalGatewayRouteTablesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2LocalGatewayRouteTablesTimeoutsElRef {
        DataEc2LocalGatewayRouteTablesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayRouteTablesTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2LocalGatewayRouteTablesDynamic {
    filter: Option<DynamicBlock<DataEc2LocalGatewayRouteTablesFilterEl>>,
}
