use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudfrontCachePolicyData {
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
struct DataCloudfrontCachePolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfrontCachePolicyData>,
}
#[derive(Clone)]
pub struct DataCloudfrontCachePolicy(Rc<DataCloudfrontCachePolicy_>);
impl DataCloudfrontCachePolicy {
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
    ) -> ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef> {
        ListRef::new(
            self.shared().clone(),
            format!(
                "{}.parameters_in_cache_key_and_forwarded_to_origin",
                self.extract_ref()
            ),
        )
    }
}
impl Referable for DataCloudfrontCachePolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudfrontCachePolicy {}
impl ToListMappable for DataCloudfrontCachePolicy {
    type O = ListRef<DataCloudfrontCachePolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudfrontCachePolicy_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudfront_cache_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudfrontCachePolicy {
    pub tf_id: String,
}
impl BuildDataCloudfrontCachePolicy {
    pub fn build(self, stack: &mut Stack) -> DataCloudfrontCachePolicy {
        let out = DataCloudfrontCachePolicy(Rc::new(DataCloudfrontCachePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataCloudfrontCachePolicyData {
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
pub struct DataCloudfrontCachePolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontCachePolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudfrontCachePolicyRef {
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
    ) -> ListRef<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef> {
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
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
{
    type O = BlockAssignable<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
{}
impl
    BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
{
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl
    {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl { items : core :: default :: Default :: default () , }
    }
}
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef { fn new (shared : StackShared , base : String) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef { DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef { shared : shared , base : base . to_string () , } } }
impl
    DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl { # [serde (skip_serializing_if = "Option::is_none")] cookie_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cookies : Option < ListField < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl > > , }
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    #[doc = "Set the field `cookie_behavior`.\n"]
    pub fn set_cookie_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cookie_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v : impl Into < ListField < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesEl > >,
    ) -> Self {
        self.cookies = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl
{
    type O = BlockAssignable<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl
{}
impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl {
            cookie_behavior: core::default::Default::default(),
            cookies: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef {
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
    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]    pub fn cookies (& self) -> ListRef < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElCookiesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
{
    type O = BlockAssignable<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
{}
impl
    BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
{
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl
    {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl { items : core :: default :: Default :: default () , }
    }
}
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef { fn new (shared : StackShared , base : String) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef { DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef { shared : shared , base : base . to_string () , } } }
impl
    DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef
{
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl { # [serde (skip_serializing_if = "Option::is_none")] header_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] headers : Option < ListField < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl > > , }
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    #[doc = "Set the field `header_behavior`.\n"]
    pub fn set_header_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v : impl Into < ListField < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersEl > >,
    ) -> Self {
        self.headers = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl
{
    type O = BlockAssignable<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl
{}
impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl {
            header_behavior: core::default::Default::default(),
            headers: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef {
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
    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]    pub fn headers (& self) -> ListRef < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElHeadersElRef >{
        ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl { # [doc = "Set the field `items`.\n"] pub fn set_items (mut self , v : impl Into < SetField < PrimField < String > > >) -> Self { self . items = Some (v . into ()) ; self } }
impl ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl { type O = BlockAssignable < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl
{}
impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl { pub fn build (self) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl { DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl { items : core :: default :: Default :: default () , } } }
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef { fn new (shared : StackShared , base : String) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef { DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef { shared : shared , base : base . to_string () , } } }
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `items` after provisioning.\n"] pub fn items (& self) -> SetRef < PrimExpr < String > > { SetRef :: new (self . shared () . clone () , format ! ("{}.items" , self . base)) } }
#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl { # [serde (skip_serializing_if = "Option::is_none")] query_string_behavior : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] query_strings : Option < ListField < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl > > , }
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    #[doc = "Set the field `query_string_behavior`.\n"]
    pub fn set_query_string_behavior(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.query_string_behavior = Some(v.into());
        self
    }
    #[doc = "Set the field `query_strings`.\n"]
    pub fn set_query_strings(
        mut self,
        v : impl Into < ListField < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsEl > >,
    ) -> Self {
        self.query_strings = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl
{
    type O = BlockAssignable<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl
{}
impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
    pub fn build(
        self,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl
    {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl {
            query_string_behavior: core::default::Default::default(),
            query_strings: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef
{
    shared: StackShared,
    base: String,
}
impl Ref
    for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef
    {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef {
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
    #[doc = "Get a reference to the value of field `query_strings` after provisioning.\n"]    pub fn query_strings (& self) -> ListRef < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElQueryStringsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    cookies_config: Option<
        ListField<
            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_accept_encoding_brotli: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_accept_encoding_gzip: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    headers_config: Option<
        ListField<
            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_strings_config: Option<
        ListField<
            DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl,
        >,
    >,
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    #[doc = "Set the field `cookies_config`.\n"]
    pub fn set_cookies_config(
        mut self,
        v: impl Into<
            ListField<
                DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigEl,
            >,
        >,
    ) -> Self {
        self.cookies_config = Some(v.into());
        self
    }
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
    #[doc = "Set the field `headers_config`.\n"]
    pub fn set_headers_config(
        mut self,
        v: impl Into<
            ListField<
                DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigEl,
            >,
        >,
    ) -> Self {
        self.headers_config = Some(v.into());
        self
    }
    #[doc = "Set the field `query_strings_config`.\n"]
    pub fn set_query_strings_config(
        mut self,
        v : impl Into < ListField < DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigEl > >,
    ) -> Self {
        self.query_strings_config = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    type O = BlockAssignable<DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {}
impl BuildDataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
    pub fn build(self) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginEl {
            cookies_config: core::default::Default::default(),
            enable_accept_encoding_brotli: core::default::Default::default(),
            enable_accept_encoding_gzip: core::default::Default::default(),
            headers_config: core::default::Default::default(),
            query_strings_config: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cookies_config` after provisioning.\n"]
    pub fn cookies_config(
        &self,
    ) -> ListRef<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElCookiesConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cookies_config", self.base),
        )
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
    #[doc = "Get a reference to the value of field `headers_config` after provisioning.\n"]
    pub fn headers_config(
        &self,
    ) -> ListRef<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElHeadersConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.headers_config", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_strings_config` after provisioning.\n"]
    pub fn query_strings_config(
        &self,
    ) -> ListRef<
        DataCloudfrontCachePolicyParametersInCacheKeyAndForwardedToOriginElQueryStringsConfigElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_strings_config", self.base),
        )
    }
}
