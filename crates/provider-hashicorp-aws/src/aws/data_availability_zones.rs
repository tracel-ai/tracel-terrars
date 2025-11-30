use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataAvailabilityZonesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_availability_zones: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_names: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_zone_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataAvailabilityZonesFilterEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataAvailabilityZonesTimeoutsEl>,
    dynamic: DataAvailabilityZonesDynamic,
}

struct DataAvailabilityZones_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAvailabilityZonesData>,
}

#[derive(Clone)]
pub struct DataAvailabilityZones(Rc<DataAvailabilityZones_>);

impl DataAvailabilityZones {
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

    #[doc = "Set the field `all_availability_zones`.\n"]
    pub fn set_all_availability_zones(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().all_availability_zones = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_names`.\n"]
    pub fn set_exclude_names(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().exclude_names = Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_zone_ids`.\n"]
    pub fn set_exclude_zone_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().exclude_zone_ids = Some(v.into());
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

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().state = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataAvailabilityZonesFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataAvailabilityZonesTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `all_availability_zones` after provisioning.\n"]
    pub fn all_availability_zones(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_availability_zones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exclude_names` after provisioning.\n"]
    pub fn exclude_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exclude_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exclude_zone_ids` after provisioning.\n"]
    pub fn exclude_zone_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exclude_zone_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `group_names` after provisioning.\n"]
    pub fn group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.group_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zone_ids` after provisioning.\n"]
    pub fn zone_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataAvailabilityZonesTimeoutsElRef {
        DataAvailabilityZonesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl Referable for DataAvailabilityZones {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataAvailabilityZones {}

impl ToListMappable for DataAvailabilityZones {
    type O = ListRef<DataAvailabilityZonesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAvailabilityZones_ {
    fn extract_datasource_type(&self) -> String {
        "aws_availability_zones".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAvailabilityZones {
    pub tf_id: String,
}

impl BuildDataAvailabilityZones {
    pub fn build(self, stack: &mut Stack) -> DataAvailabilityZones {
        let out = DataAvailabilityZones(Rc::new(DataAvailabilityZones_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAvailabilityZonesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                all_availability_zones: core::default::Default::default(),
                exclude_names: core::default::Default::default(),
                exclude_zone_ids: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                state: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAvailabilityZonesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAvailabilityZonesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataAvailabilityZonesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `all_availability_zones` after provisioning.\n"]
    pub fn all_availability_zones(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.all_availability_zones", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exclude_names` after provisioning.\n"]
    pub fn exclude_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exclude_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `exclude_zone_ids` after provisioning.\n"]
    pub fn exclude_zone_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.exclude_zone_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `group_names` after provisioning.\n"]
    pub fn group_names(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.group_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `names` after provisioning.\n"]
    pub fn names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.state", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `zone_ids` after provisioning.\n"]
    pub fn zone_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.zone_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataAvailabilityZonesTimeoutsElRef {
        DataAvailabilityZonesTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataAvailabilityZonesFilterEl {
    name: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataAvailabilityZonesFilterEl {}

impl ToListMappable for DataAvailabilityZonesFilterEl {
    type O = BlockAssignable<DataAvailabilityZonesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAvailabilityZonesFilterEl {
    #[doc = ""]
    pub name: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataAvailabilityZonesFilterEl {
    pub fn build(self) -> DataAvailabilityZonesFilterEl {
        DataAvailabilityZonesFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataAvailabilityZonesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAvailabilityZonesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataAvailabilityZonesFilterElRef {
        DataAvailabilityZonesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAvailabilityZonesFilterElRef {
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
pub struct DataAvailabilityZonesTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataAvailabilityZonesTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataAvailabilityZonesTimeoutsEl {
    type O = BlockAssignable<DataAvailabilityZonesTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAvailabilityZonesTimeoutsEl {}

impl BuildDataAvailabilityZonesTimeoutsEl {
    pub fn build(self) -> DataAvailabilityZonesTimeoutsEl {
        DataAvailabilityZonesTimeoutsEl {
            read: core::default::Default::default(),
        }
    }
}

pub struct DataAvailabilityZonesTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAvailabilityZonesTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataAvailabilityZonesTimeoutsElRef {
        DataAvailabilityZonesTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAvailabilityZonesTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAvailabilityZonesDynamic {
    filter: Option<DynamicBlock<DataAvailabilityZonesFilterEl>>,
}
