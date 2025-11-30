use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataEmrReleaseLabelsData {
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
    filters: Option<Vec<DataEmrReleaseLabelsFiltersEl>>,
    dynamic: DataEmrReleaseLabelsDynamic,
}

struct DataEmrReleaseLabels_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEmrReleaseLabelsData>,
}

#[derive(Clone)]
pub struct DataEmrReleaseLabels(Rc<DataEmrReleaseLabels_>);

impl DataEmrReleaseLabels {
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

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(self, v: impl Into<BlockAssignable<DataEmrReleaseLabelsFiltersEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = Some(d);
            }
        }
        self
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

    #[doc = "Get a reference to the value of field `release_labels` after provisioning.\n"]
    pub fn release_labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.release_labels", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<DataEmrReleaseLabelsFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }
}

impl Referable for DataEmrReleaseLabels {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataEmrReleaseLabels {}

impl ToListMappable for DataEmrReleaseLabels {
    type O = ListRef<DataEmrReleaseLabelsRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEmrReleaseLabels_ {
    fn extract_datasource_type(&self) -> String {
        "aws_emr_release_labels".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEmrReleaseLabels {
    pub tf_id: String,
}

impl BuildDataEmrReleaseLabels {
    pub fn build(self, stack: &mut Stack) -> DataEmrReleaseLabels {
        let out = DataEmrReleaseLabels(Rc::new(DataEmrReleaseLabels_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEmrReleaseLabelsData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                filters: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEmrReleaseLabelsRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrReleaseLabelsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataEmrReleaseLabelsRef {
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

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `release_labels` after provisioning.\n"]
    pub fn release_labels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.release_labels", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> ListRef<DataEmrReleaseLabelsFiltersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.filters", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEmrReleaseLabelsFiltersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
}

impl DataEmrReleaseLabelsFiltersEl {
    #[doc = "Set the field `application`.\n"]
    pub fn set_application(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application = Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataEmrReleaseLabelsFiltersEl {
    type O = BlockAssignable<DataEmrReleaseLabelsFiltersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEmrReleaseLabelsFiltersEl {}

impl BuildDataEmrReleaseLabelsFiltersEl {
    pub fn build(self) -> DataEmrReleaseLabelsFiltersEl {
        DataEmrReleaseLabelsFiltersEl {
            application: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

pub struct DataEmrReleaseLabelsFiltersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataEmrReleaseLabelsFiltersElRef {
    fn new(shared: StackShared, base: String) -> DataEmrReleaseLabelsFiltersElRef {
        DataEmrReleaseLabelsFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEmrReleaseLabelsFiltersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application` after provisioning.\n"]
    pub fn application(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEmrReleaseLabelsDynamic {
    filters: Option<DynamicBlock<DataEmrReleaseLabelsFiltersEl>>,
}
