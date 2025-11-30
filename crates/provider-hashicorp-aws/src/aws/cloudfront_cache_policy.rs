use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct CloudfrontCachePolicyData {
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
    default_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ttl: Option<PrimField<f64>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters_in_cache_key_and_forwarded_to_origin:
        Option<Vec<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl>>,
    dynamic: CloudfrontCachePolicyDynamic,
}

struct CloudfrontCachePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontCachePolicyData>,
}

#[derive(Clone)]
pub struct CloudfrontCachePolicy(Rc<CloudfrontCachePolicy_>);

impl CloudfrontCachePolicy {
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

    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `min_ttl`.\n"]
    pub fn set_min_ttl(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().min_ttl = Some(v.into());
        self
    }

    #[doc = "Set the field `parameters_in_cache_key_and_forwarded_to_origin`.\n"]
    pub fn set_parameters_in_cache_key_and_forwarded_to_origin(
        self,
        v: impl Into<BlockAssignable<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0
                    .data
                    .borrow_mut()
                    .parameters_in_cache_key_and_forwarded_to_origin = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0
                    .data
                    .borrow_mut()
                    .dynamic
                    .parameters_in_cache_key_and_forwarded_to_origin = Some(d);
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

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_ttl", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_ttl", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_ttl", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters_in_cache_key_and_forwarded_to_origin` after provisioning.\n"]
    pub fn parameters_in_cache_key_and_forwarded_to_origin(
        &self,
    ) -> ListRef<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.parameters_in_cache_key_and_forwarded_to_origin",
                self.extract_ref()
            ),
        )
    }
}

impl Referable for CloudfrontCachePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for CloudfrontCachePolicy {}

impl ToListMappable for CloudfrontCachePolicy {
    type O = ListRef<CloudfrontCachePolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontCachePolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_cache_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontCachePolicy {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCloudfrontCachePolicy {
    pub fn build(self, stack: &mut Stack) -> CloudfrontCachePolicy {
        let out = CloudfrontCachePolicy(Rc::new(CloudfrontCachePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontCachePolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                comment: core::default::Default::default(),
                default_ttl: core::default::Default::default(),
                id: core::default::Default::default(),
                max_ttl: core::default::Default::default(),
                min_ttl: core::default::Default::default(),
                name: self.name,
                parameters_in_cache_key_and_forwarded_to_origin: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontCachePolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontCachePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl CloudfrontCachePolicyRef {
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

    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_ttl", self.extract_ref()),
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

    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_ttl", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `min_ttl` after provisioning.\n"]
    pub fn min_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_ttl", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `parameters_in_cache_key_and_forwarded_to_origin` after provisioning.\n"]
    pub fn parameters_in_cache_key_and_forwarded_to_origin(
        &self,
    ) -> ListRef<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.parameters_in_cache_key_and_forwarded_to_origin",
                self.extract_ref()
            ),
        )
    }
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
{
    type O = BlockAssignable<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
{}

impl BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
    pub fn build(
        self,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
    {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef
    {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElDynamic {
    cookies: Option<
        DynamicBlock<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    cookie_behavior: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies: Option<
        Vec<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl,
        >,
    >,
    dynamic: CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElDynamic,
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl,
                        >,
                    >,
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

impl ToListMappable
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl
{
    type O = BlockAssignable<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    #[doc = ""]
    pub cookie_behavior: PrimField<String>,
}

impl BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    pub fn build(
        self,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
            cookie_behavior: self.cookie_behavior,
            cookies: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
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
    pub fn cookies(
        &self,
    ) -> ListRef<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
{
    type O = BlockAssignable<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
{}

impl BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
    pub fn build(
        self,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
    {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef
{
    shared: StackShared,
    base: String,
}

impl Ref
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef
    {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElDynamic {
    headers: Option<
        DynamicBlock<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_behavior: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<
        Vec<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl,
        >,
    >,
    dynamic: CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElDynamic,
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_behavior = Some(v.into());
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl,
                        >,
                    >,
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

impl ToListMappable
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl
{
    type O = BlockAssignable<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {}

impl BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    pub fn build(
        self,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
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
    pub fn headers(
        &self,
    ) -> ListRef<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef,
    > {
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
    type O =
        BlockAssignable<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl,
        >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl
{}

impl BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
    pub fn build(
        self,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef
{
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElDynamic {
    query_strings: Option<
        DynamicBlock<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    query_string_behavior: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_strings: Option<
        Vec<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl>,
    >,
    dynamic: CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElDynamic,
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl,
                        >,
                    >,
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

impl ToListMappable
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl
{
    type O = BlockAssignable<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl,
    >;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl
{
    #[doc = ""]
    pub query_string_behavior: PrimField<String>,
}

impl BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    pub fn build(
        self,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
            query_string_behavior: self.query_string_behavior,
            query_strings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref
    for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef
    {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
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
    ) -> ListRef<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef,
    >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElDynamic {
    cookies_config: Option<
        DynamicBlock<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl,
        >,
    >,
    headers_config: Option<
        DynamicBlock<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl,
        >,
    >,
    query_strings_config: Option<
        DynamicBlock<
            CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_accept_encoding_brotli: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_accept_encoding_gzip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies_config:
        Option<Vec<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers_config:
        Option<Vec<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_strings_config: Option<
        Vec<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl>,
    >,
    dynamic: CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElDynamic,
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    #[doc = "Set the field `enable_accept_encoding_brotli`.\n"]
    pub fn set_enable_accept_encoding_brotli(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_accept_encoding_brotli = Some(v.into());
        self
    }

    #[doc = "Set the field `enable_accept_encoding_gzip`.\n"]
    pub fn set_enable_accept_encoding_gzip(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable_accept_encoding_gzip = Some(v.into());
        self
    }

    #[doc = "Set the field `cookies_config`.\n"]
    pub fn set_cookies_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cookies_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cookies_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `headers_config`.\n"]
    pub fn set_headers_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.headers_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.headers_config = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `query_strings_config`.\n"]
    pub fn set_query_strings_config(
        mut self,
        v: impl Into<
            BlockAssignable<
                CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.query_strings_config = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.query_strings_config = Some(d);
            }
        }
        self
    }
}

impl ToListMappable for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    type O = BlockAssignable<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {}

impl BuildCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    pub fn build(self) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
            enable_accept_encoding_brotli: core::default::Default::default(),
            enable_accept_encoding_gzip: core::default::Default::default(),
            cookies_config: core::default::Default::default(),
            headers_config: core::default::Default::default(),
            query_strings_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_accept_encoding_brotli` after provisioning.\n"]
    pub fn enable_accept_encoding_brotli(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_accept_encoding_brotli", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `enable_accept_encoding_gzip` after provisioning.\n"]
    pub fn enable_accept_encoding_gzip(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_accept_encoding_gzip", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(
        &self,
    ) -> ListRef<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cookies_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(
        &self,
    ) -> ListRef<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.headers_config", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> ListRef<
        CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings_config", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudfrontCachePolicyDynamic {
    parameters_in_cache_key_and_forwarded_to_origin:
        Option<DynamicBlock<CloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl>>,
}
