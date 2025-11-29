use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSsmPatchBaselinesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_baselines: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<DataSsmPatchBaselinesFilterEl>>,
    dynamic: DataSsmPatchBaselinesDynamic,
}

struct DataSsmPatchBaselines_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsmPatchBaselinesData>,
}

#[derive(Clone)]
pub struct DataSsmPatchBaselines(Rc<DataSsmPatchBaselines_>);

impl DataSsmPatchBaselines {
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

    #[doc = "Set the field `default_baselines`.\n"]
    pub fn set_default_baselines(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_baselines = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl Into<BlockAssignable<DataSsmPatchBaselinesFilterEl>>) -> Self {
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

    #[doc = "Get a reference to the value of field `baseline_identities` after provisioning.\n"]
    pub fn baseline_identities(&self) -> ListRef<DataSsmPatchBaselinesBaselineIdentitiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.baseline_identities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_baselines` after provisioning.\n"]
    pub fn default_baselines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_baselines", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataSsmPatchBaselinesFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl Referable for DataSsmPatchBaselines {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataSsmPatchBaselines { }

impl ToListMappable for DataSsmPatchBaselines {
    type O = ListRef<DataSsmPatchBaselinesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsmPatchBaselines_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssm_patch_baselines".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsmPatchBaselines {
    pub tf_id: String,
}

impl BuildDataSsmPatchBaselines {
    pub fn build(self, stack: &mut Stack) -> DataSsmPatchBaselines {
        let out = DataSsmPatchBaselines(Rc::new(DataSsmPatchBaselines_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsmPatchBaselinesData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                default_baselines: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsmPatchBaselinesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmPatchBaselinesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataSsmPatchBaselinesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `baseline_identities` after provisioning.\n"]
    pub fn baseline_identities(&self) -> ListRef<DataSsmPatchBaselinesBaselineIdentitiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.baseline_identities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_baselines` after provisioning.\n"]
    pub fn default_baselines(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_baselines", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<DataSsmPatchBaselinesFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselinesBaselineIdentitiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    baseline_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_baseline: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operating_system: Option<PrimField<String>>,
}

impl DataSsmPatchBaselinesBaselineIdentitiesEl {
    #[doc = "Set the field `baseline_description`.\n"]
    pub fn set_baseline_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.baseline_description = Some(v.into());
        self
    }

    #[doc = "Set the field `baseline_id`.\n"]
    pub fn set_baseline_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.baseline_id = Some(v.into());
        self
    }

    #[doc = "Set the field `baseline_name`.\n"]
    pub fn set_baseline_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.baseline_name = Some(v.into());
        self
    }

    #[doc = "Set the field `default_baseline`.\n"]
    pub fn set_default_baseline(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.default_baseline = Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.operating_system = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsmPatchBaselinesBaselineIdentitiesEl {
    type O = BlockAssignable<DataSsmPatchBaselinesBaselineIdentitiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselinesBaselineIdentitiesEl {}

impl BuildDataSsmPatchBaselinesBaselineIdentitiesEl {
    pub fn build(self) -> DataSsmPatchBaselinesBaselineIdentitiesEl {
        DataSsmPatchBaselinesBaselineIdentitiesEl {
            baseline_description: core::default::Default::default(),
            baseline_id: core::default::Default::default(),
            baseline_name: core::default::Default::default(),
            default_baseline: core::default::Default::default(),
            operating_system: core::default::Default::default(),
        }
    }
}

pub struct DataSsmPatchBaselinesBaselineIdentitiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmPatchBaselinesBaselineIdentitiesElRef {
    fn new(shared: StackShared, base: String) -> DataSsmPatchBaselinesBaselineIdentitiesElRef {
        DataSsmPatchBaselinesBaselineIdentitiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselinesBaselineIdentitiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `baseline_description` after provisioning.\n"]
    pub fn baseline_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.baseline_description", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_id` after provisioning.\n"]
    pub fn baseline_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.baseline_id", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_name` after provisioning.\n"]
    pub fn baseline_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.baseline_name", self.base))
    }

    #[doc = "Get a reference to the value of field `default_baseline` after provisioning.\n"]
    pub fn default_baseline(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_baseline", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsmPatchBaselinesFilterEl {
    key: PrimField<String>,
    values: SetField<PrimField<String>>,
}

impl DataSsmPatchBaselinesFilterEl { }

impl ToListMappable for DataSsmPatchBaselinesFilterEl {
    type O = BlockAssignable<DataSsmPatchBaselinesFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsmPatchBaselinesFilterEl {
    #[doc = ""]
    pub key: PrimField<String>,
    #[doc = ""]
    pub values: SetField<PrimField<String>>,
}

impl BuildDataSsmPatchBaselinesFilterEl {
    pub fn build(self) -> DataSsmPatchBaselinesFilterEl {
        DataSsmPatchBaselinesFilterEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct DataSsmPatchBaselinesFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsmPatchBaselinesFilterElRef {
    fn new(shared: StackShared, base: String) -> DataSsmPatchBaselinesFilterElRef {
        DataSsmPatchBaselinesFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsmPatchBaselinesFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataSsmPatchBaselinesDynamic {
    filter: Option<DynamicBlock<DataSsmPatchBaselinesFilterEl>>,
}
