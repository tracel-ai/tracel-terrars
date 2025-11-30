use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataResourceexplorer2SearchData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    query_string: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view_arn: Option<PrimField<String>>,
}

struct DataResourceexplorer2Search_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataResourceexplorer2SearchData>,
}

#[derive(Clone)]
pub struct DataResourceexplorer2Search(Rc<DataResourceexplorer2Search_>);

impl DataResourceexplorer2Search {
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

    #[doc = "Set the field `view_arn`.\n"]
    pub fn set_view_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().view_arn = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_count` after provisioning.\n"]
    pub fn resource_count(&self) -> ListRef<DataResourceexplorer2SearchResourceCountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<DataResourceexplorer2SearchResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `view_arn` after provisioning.\n"]
    pub fn view_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_arn", self.extract_ref()),
        )
    }
}

impl Referable for DataResourceexplorer2Search {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataResourceexplorer2Search {}

impl ToListMappable for DataResourceexplorer2Search {
    type O = ListRef<DataResourceexplorer2SearchRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataResourceexplorer2Search_ {
    fn extract_datasource_type(&self) -> String {
        "aws_resourceexplorer2_search".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataResourceexplorer2Search {
    pub tf_id: String,
    #[doc = ""]
    pub query_string: PrimField<String>,
}

impl BuildDataResourceexplorer2Search {
    pub fn build(self, stack: &mut Stack) -> DataResourceexplorer2Search {
        let out = DataResourceexplorer2Search(Rc::new(DataResourceexplorer2Search_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataResourceexplorer2SearchData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                query_string: self.query_string,
                region: core::default::Default::default(),
                view_arn: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataResourceexplorer2SearchRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourceexplorer2SearchRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataResourceexplorer2SearchRef {
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

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_count` after provisioning.\n"]
    pub fn resource_count(&self) -> ListRef<DataResourceexplorer2SearchResourceCountElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_count", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> ListRef<DataResourceexplorer2SearchResourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resources", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `view_arn` after provisioning.\n"]
    pub fn view_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_arn", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataResourceexplorer2SearchResourceCountEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    complete: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    total_resources: Option<PrimField<f64>>,
}

impl DataResourceexplorer2SearchResourceCountEl {
    #[doc = "Set the field `complete`.\n"]
    pub fn set_complete(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.complete = Some(v.into());
        self
    }

    #[doc = "Set the field `total_resources`.\n"]
    pub fn set_total_resources(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.total_resources = Some(v.into());
        self
    }
}

impl ToListMappable for DataResourceexplorer2SearchResourceCountEl {
    type O = BlockAssignable<DataResourceexplorer2SearchResourceCountEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceexplorer2SearchResourceCountEl {}

impl BuildDataResourceexplorer2SearchResourceCountEl {
    pub fn build(self) -> DataResourceexplorer2SearchResourceCountEl {
        DataResourceexplorer2SearchResourceCountEl {
            complete: core::default::Default::default(),
            total_resources: core::default::Default::default(),
        }
    }
}

pub struct DataResourceexplorer2SearchResourceCountElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourceexplorer2SearchResourceCountElRef {
    fn new(shared: StackShared, base: String) -> DataResourceexplorer2SearchResourceCountElRef {
        DataResourceexplorer2SearchResourceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceexplorer2SearchResourceCountElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `complete` after provisioning.\n"]
    pub fn complete(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.complete", self.base))
    }

    #[doc = "Get a reference to the value of field `total_resources` after provisioning.\n"]
    pub fn total_resources(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.total_resources", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataResourceexplorer2SearchResourcesElPropertiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_reported_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataResourceexplorer2SearchResourcesElPropertiesEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }

    #[doc = "Set the field `last_reported_at`.\n"]
    pub fn set_last_reported_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_reported_at = Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataResourceexplorer2SearchResourcesElPropertiesEl {
    type O = BlockAssignable<DataResourceexplorer2SearchResourcesElPropertiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceexplorer2SearchResourcesElPropertiesEl {}

impl BuildDataResourceexplorer2SearchResourcesElPropertiesEl {
    pub fn build(self) -> DataResourceexplorer2SearchResourcesElPropertiesEl {
        DataResourceexplorer2SearchResourcesElPropertiesEl {
            data: core::default::Default::default(),
            last_reported_at: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataResourceexplorer2SearchResourcesElPropertiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourceexplorer2SearchResourcesElPropertiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataResourceexplorer2SearchResourcesElPropertiesElRef {
        DataResourceexplorer2SearchResourcesElPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceexplorer2SearchResourcesElPropertiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `last_reported_at` after provisioning.\n"]
    pub fn last_reported_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_reported_at", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataResourceexplorer2SearchResourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_reported_at: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owning_account_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<ListField<DataResourceexplorer2SearchResourcesElPropertiesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<PrimField<String>>,
}

impl DataResourceexplorer2SearchResourcesEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.arn = Some(v.into());
        self
    }

    #[doc = "Set the field `last_reported_at`.\n"]
    pub fn set_last_reported_at(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.last_reported_at = Some(v.into());
        self
    }

    #[doc = "Set the field `owning_account_id`.\n"]
    pub fn set_owning_account_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.owning_account_id = Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl Into<ListField<DataResourceexplorer2SearchResourcesElPropertiesEl>>,
    ) -> Self {
        self.properties = Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.resource_type = Some(v.into());
        self
    }

    #[doc = "Set the field `service`.\n"]
    pub fn set_service(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.service = Some(v.into());
        self
    }
}

impl ToListMappable for DataResourceexplorer2SearchResourcesEl {
    type O = BlockAssignable<DataResourceexplorer2SearchResourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataResourceexplorer2SearchResourcesEl {}

impl BuildDataResourceexplorer2SearchResourcesEl {
    pub fn build(self) -> DataResourceexplorer2SearchResourcesEl {
        DataResourceexplorer2SearchResourcesEl {
            arn: core::default::Default::default(),
            last_reported_at: core::default::Default::default(),
            owning_account_id: core::default::Default::default(),
            properties: core::default::Default::default(),
            region: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            service: core::default::Default::default(),
        }
    }
}

pub struct DataResourceexplorer2SearchResourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataResourceexplorer2SearchResourcesElRef {
    fn new(shared: StackShared, base: String) -> DataResourceexplorer2SearchResourcesElRef {
        DataResourceexplorer2SearchResourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataResourceexplorer2SearchResourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `last_reported_at` after provisioning.\n"]
    pub fn last_reported_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.last_reported_at", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `owning_account_id` after provisioning.\n"]
    pub fn owning_account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.owning_account_id", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> ListRef<DataResourceexplorer2SearchResourcesElPropertiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_type", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service", self.base))
    }
}
