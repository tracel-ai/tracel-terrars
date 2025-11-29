use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudfrontOriginAccessControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    id: PrimField<String>,
}

struct DataCloudfrontOriginAccessControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfrontOriginAccessControlData>,
}

#[derive(Clone)]
pub struct DataCloudfrontOriginAccessControl(Rc<DataCloudfrontOriginAccessControl_>);

impl DataCloudfrontOriginAccessControl {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_access_control_origin_type` after provisioning.\n"]
    pub fn origin_access_control_origin_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_access_control_origin_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_behavior` after provisioning.\n"]
    pub fn signing_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_protocol` after provisioning.\n"]
    pub fn signing_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_protocol", self.extract_ref()))
    }
}

impl Referable for DataCloudfrontOriginAccessControl {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataCloudfrontOriginAccessControl { }

impl ToListMappable for DataCloudfrontOriginAccessControl {
    type O = ListRef<DataCloudfrontOriginAccessControlRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudfrontOriginAccessControl_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudfront_origin_access_control".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontOriginAccessControl {
    pub tf_id: String,
    #[doc = ""]
    pub id: PrimField<String>,
}

impl BuildDataCloudfrontOriginAccessControl {
    pub fn build(self, stack: &mut Stack) -> DataCloudfrontOriginAccessControl {
        let out = DataCloudfrontOriginAccessControl(Rc::new(DataCloudfrontOriginAccessControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudfrontOriginAccessControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: self.id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontOriginAccessControlRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginAccessControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataCloudfrontOriginAccessControlRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `origin_access_control_origin_type` after provisioning.\n"]
    pub fn origin_access_control_origin_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_access_control_origin_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_behavior` after provisioning.\n"]
    pub fn signing_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_protocol` after provisioning.\n"]
    pub fn signing_protocol(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signing_protocol", self.extract_ref()))
    }
}
