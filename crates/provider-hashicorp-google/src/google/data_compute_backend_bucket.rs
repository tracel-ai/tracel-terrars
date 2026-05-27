use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataComputeBackendBucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
}
struct DataComputeBackendBucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataComputeBackendBucketData>,
}
#[derive(Clone)]
pub struct DataComputeBackendBucket(Rc<DataComputeBackendBucket_>);
impl DataComputeBackendBucket {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(&self, provider: &ProviderGoogle) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this Backend Bucket."]
    pub fn cdn_policy(&self) -> ListRef<DataComputeBackendBucketCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied responses."]
    pub fn custom_response_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_response_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource; provided by the\nclient when the resource is created."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe security policy associated with this backend bucket."]
    pub fn edge_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendBucket.\nNote: This cannot be set to true when loadBalancingScheme is set to INTERNAL_MANAGED."]
    pub fn enable_cdn(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nThe value can only be INTERNAL_MANAGED for cross-region internal layer 7 load balancer.\nIf loadBalancingScheme is not specified, the backend bucket can be used by classic global external load balancers, or global application external load balancers, or both.\nImportant: CDN cannot be enabled (enableCdn cannot be set to true) when loadBalancingScheme is set to INTERNAL_MANAGED. Possible values: [\"INTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeBackendBucketParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
}
impl Referable for DataComputeBackendBucket {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataComputeBackendBucket {}
impl ToListMappable for DataComputeBackendBucket {
    type O = ListRef<DataComputeBackendBucketRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataComputeBackendBucket_ {
    fn extract_datasource_type(&self) -> String {
        "google_compute_backend_bucket".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataComputeBackendBucket {
    pub tf_id: String,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildDataComputeBackendBucket {
    pub fn build(self, stack: &mut Stack) -> DataComputeBackendBucket {
        let out = DataComputeBackendBucket(Rc::new(DataComputeBackendBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataComputeBackendBucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}
pub struct DataComputeBackendBucketRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendBucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataComputeBackendBucketRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\nCloud CDN configuration for this Backend Bucket."]
    pub fn cdn_policy(&self) -> ListRef<DataComputeBackendBucketCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `compression_mode` after provisioning.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn compression_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.compression_mode", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `creation_timestamp` after provisioning.\nCreation timestamp in RFC3339 text format."]
    pub fn creation_timestamp(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.creation_timestamp", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_response_headers` after provisioning.\nHeaders that the HTTP/S load balancer should add to proxied responses."]
    pub fn custom_response_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_response_headers", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nAn optional textual description of the resource; provided by the\nclient when the resource is created."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `edge_security_policy` after provisioning.\nThe security policy associated with this backend bucket."]
    pub fn edge_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.edge_security_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `enable_cdn` after provisioning.\nIf true, enable Cloud CDN for this BackendBucket.\nNote: This cannot be set to true when loadBalancingScheme is set to INTERNAL_MANAGED."]
    pub fn enable_cdn(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_cdn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `load_balancing_scheme` after provisioning.\nThe value can only be INTERNAL_MANAGED for cross-region internal layer 7 load balancer.\nIf loadBalancingScheme is not specified, the backend bucket can be used by classic global external load balancers, or global application external load balancers, or both.\nImportant: CDN cannot be enabled (enableCdn cannot be set to true) when loadBalancingScheme is set to INTERNAL_MANAGED. Possible values: [\"INTERNAL_MANAGED\"]"]
    pub fn load_balancing_scheme(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.load_balancing_scheme", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\nAdditional params passed with the request, but not persisted as part of resource payload"]
    pub fn params(&self) -> ListRef<DataComputeBackendBucketParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `self_link` after provisioning.\n"]
    pub fn self_link(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.self_link", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
}
impl DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[doc = "Set the field `header_name`.\n"]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    type O = BlockAssignable<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {}
impl BuildDataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
        DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
            header_name: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
        DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_http_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_whitelist: Option<ListField<PrimField<String>>>,
}
impl DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    #[doc = "Set the field `include_http_headers`.\n"]
    pub fn set_include_http_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_whitelist`.\n"]
    pub fn set_query_string_whitelist(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.query_string_whitelist = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    type O = BlockAssignable<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {}
impl BuildDataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
        DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
            include_http_headers: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
        DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_http_headers` after provisioning.\n"]
    pub fn include_http_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_http_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_whitelist` after provisioning.\n"]
    pub fn query_string_whitelist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_string_whitelist", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
}
impl DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    #[doc = "Set the field `code`.\n"]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    type O = BlockAssignable<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {}
impl BuildDataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
        DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
        DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendBucketCdnPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_cache_on_request_headers:
        Option<ListField<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_policy: Option<ListField<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ttl: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_caching: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_caching_policy:
        Option<ListField<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_coalescing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_while_stale: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signed_url_cache_max_age_sec: Option<PrimField<f64>>,
}
impl DataComputeBackendBucketCdnPolicyEl {
    #[doc = "Set the field `bypass_cache_on_request_headers`.\n"]
    pub fn set_bypass_cache_on_request_headers(
        mut self,
        v: impl Into<ListField<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    ) -> Self {
        self.bypass_cache_on_request_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl Into<ListField<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>>,
    ) -> Self {
        self.cache_key_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `cache_mode`.\n"]
    pub fn set_cache_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ttl`.\n"]
    pub fn set_client_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\n"]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ttl`.\n"]
    pub fn set_max_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching`.\n"]
    pub fn set_negative_caching(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negative_caching = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching_policy`.\n"]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl Into<ListField<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>>,
    ) -> Self {
        self.negative_caching_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `request_coalescing`.\n"]
    pub fn set_request_coalescing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.request_coalescing = Some(v.into());
        self
    }
    #[doc = "Set the field `serve_while_stale`.\n"]
    pub fn set_serve_while_stale(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.serve_while_stale = Some(v.into());
        self
    }
    #[doc = "Set the field `signed_url_cache_max_age_sec`.\n"]
    pub fn set_signed_url_cache_max_age_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.signed_url_cache_max_age_sec = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendBucketCdnPolicyEl {
    type O = BlockAssignable<DataComputeBackendBucketCdnPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendBucketCdnPolicyEl {}
impl BuildDataComputeBackendBucketCdnPolicyEl {
    pub fn build(self) -> DataComputeBackendBucketCdnPolicyEl {
        DataComputeBackendBucketCdnPolicyEl {
            bypass_cache_on_request_headers: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            request_coalescing: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendBucketCdnPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendBucketCdnPolicyElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendBucketCdnPolicyElRef {
        DataComputeBackendBucketCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendBucketCdnPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> ListRef<DataComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bypass_cache_on_request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(
        &self,
    ) -> ListRef<DataComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_key_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_mode` after provisioning.\n"]
    pub fn cache_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `client_ttl` after provisioning.\n"]
    pub fn client_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\n"]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\n"]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `negative_caching` after provisioning.\n"]
    pub fn negative_caching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.negative_caching", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(
        &self,
    ) -> ListRef<DataComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.negative_caching_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_coalescing` after provisioning.\n"]
    pub fn request_coalescing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_coalescing", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `serve_while_stale` after provisioning.\n"]
    pub fn serve_while_stale(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serve_while_stale", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_url_cache_max_age_sec` after provisioning.\n"]
    pub fn signed_url_cache_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_url_cache_max_age_sec", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataComputeBackendBucketParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl DataComputeBackendBucketParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\n"]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for DataComputeBackendBucketParamsEl {
    type O = BlockAssignable<DataComputeBackendBucketParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataComputeBackendBucketParamsEl {}
impl BuildDataComputeBackendBucketParamsEl {
    pub fn build(self) -> DataComputeBackendBucketParamsEl {
        DataComputeBackendBucketParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct DataComputeBackendBucketParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataComputeBackendBucketParamsElRef {
    fn new(shared: StackShared, base: String) -> DataComputeBackendBucketParamsElRef {
        DataComputeBackendBucketParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataComputeBackendBucketParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\n"]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
