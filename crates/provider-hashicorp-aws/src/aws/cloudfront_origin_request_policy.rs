use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct CloudfrontOriginRequestPolicyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies_config: Option<Vec<CloudfrontOriginRequestPolicyCookiesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers_config: Option<Vec<CloudfrontOriginRequestPolicyHeadersConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_strings_config: Option<Vec<CloudfrontOriginRequestPolicyQueryStringsConfigEl>>,
    dynamic: CloudfrontOriginRequestPolicyDynamic,
}
struct CloudfrontOriginRequestPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontOriginRequestPolicyData>,
}
#[derive(Clone)]
pub struct CloudfrontOriginRequestPolicy(Rc<CloudfrontOriginRequestPolicy_>);
impl CloudfrontOriginRequestPolicy {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }
    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }
    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }
    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => true,
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    }
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }
    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(r.extract_ref());
        self
    }
    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0
            .data
            .borrow_mut()
            .lifecycle
            .replace_triggered_by
            .push(attr.to_string());
        self
    }
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comment = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `cookies_config`.\n"]
    pub fn set_cookies_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cookies_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cookies_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `headers_config`.\n"]
    pub fn set_headers_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().headers_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.headers_config = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `query_strings_config`.\n"]
    pub fn set_query_strings_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().query_strings_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.query_strings_config = Some(d);
            }
        }
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> ListRef<CloudfrontOriginRequestPolicyCookiesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cookies_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> ListRef<CloudfrontOriginRequestPolicyHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.headers_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> ListRef<CloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings_config", self.extract_ref()),
        )
    }
}
impl Referable for CloudfrontOriginRequestPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for CloudfrontOriginRequestPolicy {}
impl ToListMappable for CloudfrontOriginRequestPolicy {
    type O = ListRef<CloudfrontOriginRequestPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for CloudfrontOriginRequestPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_origin_request_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildCloudfrontOriginRequestPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}
impl BuildCloudfrontOriginRequestPolicy {
    pub fn build(self, stack: &mut Stack) -> CloudfrontOriginRequestPolicy {
        let out = CloudfrontOriginRequestPolicy(Rc::new(CloudfrontOriginRequestPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontOriginRequestPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                cookies_config: core::default::Default::default(),
                headers_config: core::default::Default::default(),
                query_strings_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct CloudfrontOriginRequestPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontOriginRequestPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl CloudfrontOriginRequestPolicyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
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
    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.etag", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(&self) -> ListRef<CloudfrontOriginRequestPolicyCookiesConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cookies_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(&self) -> ListRef<CloudfrontOriginRequestPolicyHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.headers_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> ListRef<CloudfrontOriginRequestPolicyQueryStringsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    type O = BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {}
impl BuildCloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
        CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
        CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyCookiesConfigElDynamic {
    cookies: Option<DynamicBlock<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
}
#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyCookiesConfigEl {
    cookie_behavior: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<Vec<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
    dynamic: CloudfrontOriginRequestPolicyCookiesConfigElDynamic,
}
impl CloudfrontOriginRequestPolicyCookiesConfigEl {
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigElCookiesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cookies = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cookies = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontOriginRequestPolicyCookiesConfigEl {
    type O = BlockAssignable<CloudfrontOriginRequestPolicyCookiesConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontOriginRequestPolicyCookiesConfigEl {
    #[doc = ""]
    pub cookie_behavior: PrimField<String>,
}
impl BuildCloudfrontOriginRequestPolicyCookiesConfigEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyCookiesConfigEl {
        CloudfrontOriginRequestPolicyCookiesConfigEl {
            cookie_behavior: self.cookie_behavior,
            cookies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontOriginRequestPolicyCookiesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontOriginRequestPolicyCookiesConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontOriginRequestPolicyCookiesConfigElRef {
        CloudfrontOriginRequestPolicyCookiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontOriginRequestPolicyCookiesConfigElRef {
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
    pub fn cookies(&self) -> ListRef<CloudfrontOriginRequestPolicyCookiesConfigElCookiesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    type O = BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {}
impl BuildCloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
        CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
        CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyHeadersConfigElDynamic {
    headers: Option<DynamicBlock<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
}
#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<Vec<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
    dynamic: CloudfrontOriginRequestPolicyHeadersConfigElDynamic,
}
impl CloudfrontOriginRequestPolicyHeadersConfigEl {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigElHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontOriginRequestPolicyHeadersConfigEl {
    type O = BlockAssignable<CloudfrontOriginRequestPolicyHeadersConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontOriginRequestPolicyHeadersConfigEl {}
impl BuildCloudfrontOriginRequestPolicyHeadersConfigEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyHeadersConfigEl {
        CloudfrontOriginRequestPolicyHeadersConfigEl {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontOriginRequestPolicyHeadersConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontOriginRequestPolicyHeadersConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontOriginRequestPolicyHeadersConfigElRef {
        CloudfrontOriginRequestPolicyHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontOriginRequestPolicyHeadersConfigElRef {
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
    pub fn headers(&self) -> ListRef<CloudfrontOriginRequestPolicyHeadersConfigElHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}
#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    type O = BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {}
impl BuildCloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
        CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
        CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyQueryStringsConfigElDynamic {
    query_strings:
        Option<DynamicBlock<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
}
#[derive(Serialize)]
pub struct CloudfrontOriginRequestPolicyQueryStringsConfigEl {
    query_string_behavior: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_strings: Option<Vec<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
    dynamic: CloudfrontOriginRequestPolicyQueryStringsConfigElDynamic,
}
impl CloudfrontOriginRequestPolicyQueryStringsConfigEl {
    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_strings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_strings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for CloudfrontOriginRequestPolicyQueryStringsConfigEl {
    type O = BlockAssignable<CloudfrontOriginRequestPolicyQueryStringsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    #[doc = ""]
    pub query_string_behavior: PrimField<String>,
}
impl BuildCloudfrontOriginRequestPolicyQueryStringsConfigEl {
    pub fn build(self) -> CloudfrontOriginRequestPolicyQueryStringsConfigEl {
        CloudfrontOriginRequestPolicyQueryStringsConfigEl {
            query_string_behavior: self.query_string_behavior,
            query_strings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
        CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl CloudfrontOriginRequestPolicyQueryStringsConfigElRef {
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
    ) -> ListRef<CloudfrontOriginRequestPolicyQueryStringsConfigElQueryStringsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct CloudfrontOriginRequestPolicyDynamic {
    cookies_config: Option<DynamicBlock<CloudfrontOriginRequestPolicyCookiesConfigEl>>,
    headers_config: Option<DynamicBlock<CloudfrontOriginRequestPolicyHeadersConfigEl>>,
    query_strings_config: Option<DynamicBlock<CloudfrontOriginRequestPolicyQueryStringsConfigEl>>,
}
