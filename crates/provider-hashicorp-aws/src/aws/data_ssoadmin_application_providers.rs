use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataSsoadminApplicationProvidersData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

struct DataSsoadminApplicationProviders_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataSsoadminApplicationProvidersData>,
}

#[derive(Clone)]
pub struct DataSsoadminApplicationProviders(Rc<DataSsoadminApplicationProviders_>);

impl DataSsoadminApplicationProviders {
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

    #[doc = "Get a reference to the value of field `application_providers` after provisioning.\n"]
    pub fn application_providers(
        &self,
    ) -> ListRef<DataSsoadminApplicationProvidersApplicationProvidersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_providers", self.extract_ref()),
        )
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
}

impl Referable for DataSsoadminApplicationProviders {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataSsoadminApplicationProviders {}

impl ToListMappable for DataSsoadminApplicationProviders {
    type O = ListRef<DataSsoadminApplicationProvidersRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataSsoadminApplicationProviders_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ssoadmin_application_providers".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSsoadminApplicationProviders {
    pub tf_id: String,
}

impl BuildDataSsoadminApplicationProviders {
    pub fn build(self, stack: &mut Stack) -> DataSsoadminApplicationProviders {
        let out = DataSsoadminApplicationProviders(Rc::new(DataSsoadminApplicationProviders_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataSsoadminApplicationProvidersData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSsoadminApplicationProvidersRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationProvidersRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataSsoadminApplicationProvidersRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `application_providers` after provisioning.\n"]
    pub fn application_providers(
        &self,
    ) -> ListRef<DataSsoadminApplicationProvidersApplicationProvidersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.application_providers", self.extract_ref()),
        )
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
}

#[derive(Serialize)]
pub struct DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<PrimField<String>>,
}

impl DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.display_name = Some(v.into());
        self
    }

    #[doc = "Set the field `icon_url`.\n"]
    pub fn set_icon_url(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.icon_url = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl {
    type O = BlockAssignable<DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl {}

impl BuildDataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl {
    pub fn build(self) -> DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl {
        DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl {
            description: core::default::Default::default(),
            display_name: core::default::Default::default(),
            icon_url: core::default::Default::default(),
        }
    }
}

pub struct DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataElRef {
        DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `icon_url` after provisioning.\n"]
    pub fn icon_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.icon_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSsoadminApplicationProvidersApplicationProvidersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_provider_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_data:
        Option<ListField<DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    federation_protocol: Option<PrimField<String>>,
}

impl DataSsoadminApplicationProvidersApplicationProvidersEl {
    #[doc = "Set the field `application_provider_arn`.\n"]
    pub fn set_application_provider_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.application_provider_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `display_data`.\n"]
    pub fn set_display_data(
        mut self,
        v: impl Into<ListField<DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataEl>>,
    ) -> Self {
        self.display_data = Some(v.into());
        self
    }

    #[doc = "Set the field `federation_protocol`.\n"]
    pub fn set_federation_protocol(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.federation_protocol = Some(v.into());
        self
    }
}

impl ToListMappable for DataSsoadminApplicationProvidersApplicationProvidersEl {
    type O = BlockAssignable<DataSsoadminApplicationProvidersApplicationProvidersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSsoadminApplicationProvidersApplicationProvidersEl {}

impl BuildDataSsoadminApplicationProvidersApplicationProvidersEl {
    pub fn build(self) -> DataSsoadminApplicationProvidersApplicationProvidersEl {
        DataSsoadminApplicationProvidersApplicationProvidersEl {
            application_provider_arn: core::default::Default::default(),
            display_data: core::default::Default::default(),
            federation_protocol: core::default::Default::default(),
        }
    }
}

pub struct DataSsoadminApplicationProvidersApplicationProvidersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataSsoadminApplicationProvidersApplicationProvidersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataSsoadminApplicationProvidersApplicationProvidersElRef {
        DataSsoadminApplicationProvidersApplicationProvidersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSsoadminApplicationProvidersApplicationProvidersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_provider_arn` after provisioning.\n"]
    pub fn application_provider_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.application_provider_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `display_data` after provisioning.\n"]
    pub fn display_data(
        &self,
    ) -> ListRef<DataSsoadminApplicationProvidersApplicationProvidersElDisplayDataElRef> {
        ListRef::new(self.shared().clone(), format!("{}.display_data", self.base))
    }

    #[doc = "Get a reference to the value of field `federation_protocol` after provisioning.\n"]
    pub fn federation_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.federation_protocol", self.base),
        )
    }
}
