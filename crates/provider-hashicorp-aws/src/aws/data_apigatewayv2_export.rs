use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataApigatewayv2ExportData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    api_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_version: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_extensions: Option<PrimField<bool>>,
    output_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    specification: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage_name: Option<PrimField<String>>,
}

struct DataApigatewayv2Export_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataApigatewayv2ExportData>,
}

#[derive(Clone)]
pub struct DataApigatewayv2Export(Rc<DataApigatewayv2Export_>);

impl DataApigatewayv2Export {
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

    #[doc = "Set the field `export_version`.\n"]
    pub fn set_export_version(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().export_version = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `include_extensions`.\n"]
    pub fn set_include_extensions(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().include_extensions = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `stage_name`.\n"]
    pub fn set_stage_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stage_name = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_version` after provisioning.\n"]
    pub fn export_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_extensions` after provisioning.\n"]
    pub fn include_extensions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_extensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specification` after provisioning.\n"]
    pub fn specification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }
}

impl Referable for DataApigatewayv2Export {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataApigatewayv2Export { }

impl ToListMappable for DataApigatewayv2Export {
    type O = ListRef<DataApigatewayv2ExportRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataApigatewayv2Export_ {
    fn extract_datasource_type(&self) -> String {
        "aws_apigatewayv2_export".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataApigatewayv2Export {
    pub tf_id: String,
    #[doc = ""]
    pub api_id: PrimField<String>,
    #[doc = ""]
    pub output_type: PrimField<String>,
    #[doc = ""]
    pub specification: PrimField<String>,
}

impl BuildDataApigatewayv2Export {
    pub fn build(self, stack: &mut Stack) -> DataApigatewayv2Export {
        let out = DataApigatewayv2Export(Rc::new(DataApigatewayv2Export_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataApigatewayv2ExportData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                api_id: self.api_id,
                export_version: core::default::Default::default(),
                id: core::default::Default::default(),
                include_extensions: core::default::Default::default(),
                output_type: self.output_type,
                region: core::default::Default::default(),
                specification: self.specification,
                stage_name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataApigatewayv2ExportRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataApigatewayv2ExportRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataApigatewayv2ExportRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `export_version` after provisioning.\n"]
    pub fn export_version(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.export_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `include_extensions` after provisioning.\n"]
    pub fn include_extensions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_extensions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_type` after provisioning.\n"]
    pub fn output_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.output_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specification` after provisioning.\n"]
    pub fn specification(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }
}
