use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCloudfrontOriginAccessIdentitiesData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataCloudfrontOriginAccessIdentities_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfrontOriginAccessIdentitiesData>,
}

#[derive(Clone)]
pub struct DataCloudfrontOriginAccessIdentities(Rc<DataCloudfrontOriginAccessIdentities_>);

impl DataCloudfrontOriginAccessIdentities {
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

    #[doc = "Set the field `comments`.\n"]
    pub fn set_comments(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().comments = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\n"]
    pub fn comments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.comments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_arns` after provisioning.\n"]
    pub fn iam_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.iam_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_canonical_user_ids` after provisioning.\n"]
    pub fn s3_canonical_user_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.s3_canonical_user_ids", self.extract_ref()),
        )
    }
}

impl Referable for DataCloudfrontOriginAccessIdentities {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCloudfrontOriginAccessIdentities {}

impl ToListMappable for DataCloudfrontOriginAccessIdentities {
    type O = ListRef<DataCloudfrontOriginAccessIdentitiesRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudfrontOriginAccessIdentities_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudfront_origin_access_identities".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontOriginAccessIdentities {
    pub tf_id: String,
}

impl BuildDataCloudfrontOriginAccessIdentities {
    pub fn build(self, stack: &mut Stack) -> DataCloudfrontOriginAccessIdentities {
        let out =
            DataCloudfrontOriginAccessIdentities(Rc::new(DataCloudfrontOriginAccessIdentities_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCloudfrontOriginAccessIdentitiesData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    for_each: None,
                    comments: core::default::Default::default(),
                    id: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontOriginAccessIdentitiesRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginAccessIdentitiesRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCloudfrontOriginAccessIdentitiesRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `comments` after provisioning.\n"]
    pub fn comments(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.comments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_arns` after provisioning.\n"]
    pub fn iam_arns(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.iam_arns", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ids` after provisioning.\n"]
    pub fn ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_canonical_user_ids` after provisioning.\n"]
    pub fn s3_canonical_user_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(
            self.shared().clone(),
            format!("{}.s3_canonical_user_ids", self.extract_ref()),
        )
    }
}
