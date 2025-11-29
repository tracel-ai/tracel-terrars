use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataInstancesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_state_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataInstancesFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataInstancesTimeoutsEl>,
    dynamic: DataInstancesDynamic,
}

struct DataInstances_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataInstancesData>,
}

#[derive(Clone)]
pub struct DataInstances(Rc<DataInstances_>);

impl DataInstances {
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

    #[doc = "Set the field `instance_state_names`.\n"]
    pub fn set_instance_state_names(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().instance_state_names = Some(v.into());
        self
    }

    #[doc = "Set the field `instance_tags`.\n"]
    pub fn set_instance_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().instance_tags = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataInstancesFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl Into<DataInstancesTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `instance_state_names` after provisioning.\n"]
    pub fn instance_state_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.instance_state_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tags` after provisioning.\n"]
    pub fn instance_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.instance_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ips` after provisioning.\n"]
    pub fn public_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.public_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataInstancesTimeoutsElRef {
        DataInstancesTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DataInstances {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataInstances { }

impl ToListMappable for DataInstances {
    type O = ListRef<DataInstancesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataInstances_ {
    fn extract_datasource_type(&self) -> String {
        "aws_instances".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataInstances {
    pub tf_id: String,
}

impl BuildDataInstances {
    pub fn build(self, stack: &mut Stack) -> DataInstances {
        let out = DataInstances(Rc::new(DataInstances_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataInstancesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                instance_state_names: core::default::Default::default(),
                instance_tags: core::default::Default::default(),
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

pub struct DataInstancesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstancesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataInstancesRef {
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

    #[doc = "Get a reference to the value of field `instance_state_names` after provisioning.\n"]
    pub fn instance_state_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.instance_state_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tags` after provisioning.\n"]
    pub fn instance_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.instance_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ips` after provisioning.\n"]
    pub fn public_ips(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.public_ips", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataInstancesTimeoutsElRef {
        DataInstancesTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataInstancesFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataInstancesFilterEl { }

impl ToListMappable for DataInstancesFilterEl {
    type O = BlockAssignable<DataInstancesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstancesFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataInstancesFilterEl {
    pub fn build(self) -> DataInstancesFilterEl {
        DataInstancesFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataInstancesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstancesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataInstancesFilterElRef {
        DataInstancesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstancesFilterElRef {
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
pub struct DataInstancesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataInstancesTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataInstancesTimeoutsEl {
    type O = BlockAssignable<DataInstancesTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataInstancesTimeoutsEl {}

impl BuildDataInstancesTimeoutsEl {
    pub fn build(self) -> DataInstancesTimeoutsEl {
        DataInstancesTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataInstancesTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataInstancesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataInstancesTimeoutsElRef {
        DataInstancesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataInstancesTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataInstancesDynamic {
    filter: Option<DynamicBlock<DataInstancesFilterEl>>,
}
