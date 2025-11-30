use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DataCloudfrontOriginRequestPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

struct DataCloudfrontOriginRequestPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfrontOriginRequestPolicyData>,
}

#[derive(Clone)]
pub struct DataCloudfrontOriginRequestPolicy(Rc<DataCloudfrontOriginRequestPolicy_>);

impl DataCloudfrontOriginRequestPolicy {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> ListRef<DataCloudfrontOriginRequestPolicyCookiesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cookies_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> ListRef<DataCloudfrontOriginRequestPolicyHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.headers_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> ListRef<DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings_config", self.extract_ref()),
        )
    }
}

impl Referable for DataCloudfrontOriginRequestPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Datasource for DataCloudfrontOriginRequestPolicy {}

impl ToListMappable for DataCloudfrontOriginRequestPolicy {
    type O = ListRef<DataCloudfrontOriginRequestPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataCloudfrontOriginRequestPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudfront_origin_request_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicy {
    pub tf_id: String,
}

impl BuildDataCloudfrontOriginRequestPolicy {
    pub fn build(self, stack: &mut Stack) -> DataCloudfrontOriginRequestPolicy {
        let out = DataCloudfrontOriginRequestPolicy(Rc::new(DataCloudfrontOriginRequestPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudfrontOriginRequestPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudfrontOriginRequestPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginRequestPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DataCloudfrontOriginRequestPolicyRef {
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

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> ListRef<DataCloudfrontOriginRequestPolicyCookiesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cookies_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> ListRef<DataCloudfrontOriginRequestPolicyHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.headers_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> ListRef<DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings_config", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    type O = BlockAssignable<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {}

impl BuildDataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
        DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
        DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyCookiesConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookie_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<ListField<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigEl {
    #[doc = "Set the field `cookie_behavior`.\n"]
    pub fn set_cookie_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cookie_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl Into<ListField<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
    ) -> Self {
        self.cookies = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontOriginRequestPolicyCookiesConfigEl {
    type O = BlockAssignable<DataCloudfrontOriginRequestPolicyCookiesConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyCookiesConfigEl {}

impl BuildDataCloudfrontOriginRequestPolicyCookiesConfigEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyCookiesConfigEl {
        DataCloudfrontOriginRequestPolicyCookiesConfigEl {
            cookie_behavior: core::default::Default::default(),
            cookies: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
        DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyCookiesConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cookie_behavior` after provisioning.\n"]
    pub fn cookie_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cookie_behavior", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(&self) -> ListRef<DataCloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    type O = BlockAssignable<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {}

impl BuildDataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
        DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
        DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<ListField<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigEl {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<ListField<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
    ) -> Self {
        self.headers = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontOriginRequestPolicyHeadersConfigEl {
    type O = BlockAssignable<DataCloudfrontOriginRequestPolicyHeadersConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyHeadersConfigEl {}

impl BuildDataCloudfrontOriginRequestPolicyHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyHeadersConfigEl {
        DataCloudfrontOriginRequestPolicyHeadersConfigEl {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
        DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_behavior` after provisioning.\n"]
    pub fn header_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.header_behavior", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> ListRef<DataCloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    type O = BlockAssignable<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {}

impl BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_strings:
        Option<ListField<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    #[doc = "Set the field `query_string_behavior`.\n"]
    pub fn set_query_string_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_string_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v: impl Into<ListField<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
    ) -> Self {
        self.query_strings = Some(v.into());
        self
    }
}

impl ToListMappable for DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    type O = BlockAssignable<DataCloudfrontOriginRequestPolicyQueryStringsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigEl {}

impl BuildDataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    pub fn build(self) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigEl {
            query_string_behavior: core::default::Default::default(),
            query_strings: core::default::Default::default(),
        }
    }
}

pub struct DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
        DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_string_behavior` after provisioning.\n"]
    pub fn query_string_behavior(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.query_string_behavior", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_strings` after provisioning.\n"]
    pub fn query_strings(
        &self,
    ) -> ListRef<DataCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings", self.base),
        )
    }
}
