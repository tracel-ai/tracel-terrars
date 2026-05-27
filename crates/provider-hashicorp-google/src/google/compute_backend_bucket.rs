use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ComputeBackendBucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compression_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_response_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edge_security_policy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_cdn: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_balancing_scheme: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cdn_policy: Option<Vec<ComputeBackendBucketCdnPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Vec<ComputeBackendBucketParamsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ComputeBackendBucketTimeoutsEl>,
    dynamic: ComputeBackendBucketDynamic,
}
struct ComputeBackendBucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ComputeBackendBucketData>,
}
#[derive(Clone)]
pub struct ComputeBackendBucket(Rc<ComputeBackendBucket_>);
impl ComputeBackendBucket {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }
    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }
    pub fn set_provider(self, provider: &ProviderGoogle) -> Self {
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
    #[doc = "Set the field `compression_mode`.\nCompress text responses using Brotli or gzip compression, based on the client's Accept-Encoding header. Possible values: [\"AUTOMATIC\", \"DISABLED\"]"]
    pub fn set_compression_mode(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().compression_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `custom_response_headers`.\nHeaders that the HTTP/S load balancer should add to proxied responses."]
    pub fn set_custom_response_headers(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_response_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `description`.\nAn optional textual description of the resource; provided by the\nclient when the resource is created."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `edge_security_policy`.\nThe security policy associated with this backend bucket."]
    pub fn set_edge_security_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().edge_security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `enable_cdn`.\nIf true, enable Cloud CDN for this BackendBucket.\nNote: This cannot be set to true when loadBalancingScheme is set to INTERNAL_MANAGED."]
    pub fn set_enable_cdn(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_cdn = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `load_balancing_scheme`.\nThe value can only be INTERNAL_MANAGED for cross-region internal layer 7 load balancer.\nIf loadBalancingScheme is not specified, the backend bucket can be used by classic global external load balancers, or global application external load balancers, or both.\nImportant: CDN cannot be enabled (enableCdn cannot be set to true) when loadBalancingScheme is set to INTERNAL_MANAGED. Possible values: [\"INTERNAL_MANAGED\"]"]
    pub fn set_load_balancing_scheme(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().load_balancing_scheme = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `cdn_policy`.\n"]
    pub fn set_cdn_policy(
        self,
        v: impl Into<BlockAssignable<ComputeBackendBucketCdnPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cdn_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cdn_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `params`.\n"]
    pub fn set_params(self, v: impl Into<BlockAssignable<ComputeBackendBucketParamsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().params = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.params = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<ComputeBackendBucketTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> ListRef<ComputeBackendBucketCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeBackendBucketParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendBucketTimeoutsElRef {
        ComputeBackendBucketTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ComputeBackendBucket {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ComputeBackendBucket {}
impl ToListMappable for ComputeBackendBucket {
    type O = ListRef<ComputeBackendBucketRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ComputeBackendBucket_ {
    fn extract_resource_type(&self) -> String {
        "google_compute_backend_bucket".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildComputeBackendBucket {
    pub tf_id: String,
    #[doc = "Cloud Storage bucket name."]
    pub bucket_name: PrimField<String>,
    #[doc = "Name of the resource. Provided by the client when the resource is\ncreated. The name must be 1-63 characters long, and comply with\nRFC1035.  Specifically, the name must be 1-63 characters long and\nmatch the regular expression '[a-z]([-a-z0-9]*[a-z0-9])?' which means\nthe first character must be a lowercase letter, and all following\ncharacters must be a dash, lowercase letter, or digit, except the\nlast character, which cannot be a dash."]
    pub name: PrimField<String>,
}
impl BuildComputeBackendBucket {
    pub fn build(self, stack: &mut Stack) -> ComputeBackendBucket {
        let out = ComputeBackendBucket(Rc::new(ComputeBackendBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ComputeBackendBucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket_name: self.bucket_name,
                compression_mode: core::default::Default::default(),
                custom_response_headers: core::default::Default::default(),
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                edge_security_policy: core::default::Default::default(),
                enable_cdn: core::default::Default::default(),
                id: core::default::Default::default(),
                load_balancing_scheme: core::default::Default::default(),
                name: self.name,
                project: core::default::Default::default(),
                cdn_policy: core::default::Default::default(),
                params: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ComputeBackendBucketRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendBucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ComputeBackendBucketRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nCloud Storage bucket name."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.bucket_name", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `cdn_policy` after provisioning.\n"]
    pub fn cdn_policy(&self) -> ListRef<ComputeBackendBucketCdnPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cdn_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `params` after provisioning.\n"]
    pub fn params(&self) -> ListRef<ComputeBackendBucketParamsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.params", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ComputeBackendBucketTimeoutsElRef {
        ComputeBackendBucketTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header_name: Option<PrimField<String>>,
}
impl ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    #[doc = "Set the field `header_name`.\nThe header field name to match on when bypassing cache. Values are case-insensitive."]
    pub fn set_header_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header_name = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    type O = BlockAssignable<ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {}
impl BuildComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
    pub fn build(self) -> ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
        ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl {
            header_name: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
        ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header_name` after provisioning.\nThe header field name to match on when bypassing cache. Values are case-insensitive."]
    pub fn header_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}
#[derive(Serialize)]
pub struct ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_http_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query_string_whitelist: Option<ListField<PrimField<String>>>,
}
impl ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    #[doc = "Set the field `include_http_headers`.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn set_include_http_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.include_http_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `query_string_whitelist`.\nNames of query string parameters to include in cache keys.\nDefault parameters are always included. '&' and '=' will\nbe percent encoded and not treated as delimiters."]
    pub fn set_query_string_whitelist(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.query_string_whitelist = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    type O = BlockAssignable<ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {}
impl BuildComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
    pub fn build(self) -> ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
        ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl {
            include_http_headers: core::default::Default::default(),
            query_string_whitelist: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
        ComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `include_http_headers` after provisioning.\nAllows HTTP request headers (by name) to be used in the\ncache key."]
    pub fn include_http_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.include_http_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `query_string_whitelist` after provisioning.\nNames of query string parameters to include in cache keys.\nDefault parameters are always included. '&' and '=' will\nbe percent encoded and not treated as delimiters."]
    pub fn query_string_whitelist(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.query_string_whitelist", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<PrimField<f64>>,
}
impl ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    #[doc = "Set the field `code`.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn set_code(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.code = Some(v.into());
        self
    }
    #[doc = "Set the field `ttl`.\nThe TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s\n(30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL."]
    pub fn set_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ttl = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    type O = BlockAssignable<ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {}
impl BuildComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
    pub fn build(self) -> ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
        ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl {
            code: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
        ComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `code` after provisioning.\nThe HTTP status code to define a TTL against. Only HTTP status codes 300, 301, 308, 404, 405, 410, 421, 451 and 501\ncan be specified as values, and you cannot specify a status code more than once."]
    pub fn code(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }
    #[doc = "Get a reference to the value of field `ttl` after provisioning.\nThe TTL (in seconds) for which to cache responses with the corresponding status code. The maximum allowed value is 1800s\n(30 minutes), noting that infrequently accessed objects may be evicted from the cache before the defined TTL."]
    pub fn ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeBackendBucketCdnPolicyElDynamic {
    bypass_cache_on_request_headers:
        Option<DynamicBlock<ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    cache_key_policy: Option<DynamicBlock<ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>>,
    negative_caching_policy:
        Option<DynamicBlock<ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>>,
}
#[derive(Serialize)]
pub struct ComputeBackendBucketCdnPolicyEl {
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
    request_coalescing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serve_while_stale: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signed_url_cache_max_age_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bypass_cache_on_request_headers:
        Option<Vec<ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_key_policy: Option<Vec<ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_caching_policy: Option<Vec<ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>>,
    dynamic: ComputeBackendBucketCdnPolicyElDynamic,
}
impl ComputeBackendBucketCdnPolicyEl {
    #[doc = "Set the field `cache_mode`.\nSpecifies the cache setting for all responses from this backend.\nThe possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn set_cache_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cache_mode = Some(v.into());
        self
    }
    #[doc = "Set the field `client_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn set_client_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.client_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `default_ttl`.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age). When the 'cache_mode'\nis set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn set_default_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.default_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `max_ttl`.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn set_max_ttl(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_ttl = Some(v.into());
        self
    }
    #[doc = "Set the field `negative_caching`.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects."]
    pub fn set_negative_caching(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negative_caching = Some(v.into());
        self
    }
    #[doc = "Set the field `request_coalescing`.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests to the origin."]
    pub fn set_request_coalescing(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.request_coalescing = Some(v.into());
        self
    }
    #[doc = "Set the field `serve_while_stale`.\nServe existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache."]
    pub fn set_serve_while_stale(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.serve_while_stale = Some(v.into());
        self
    }
    #[doc = "Set the field `signed_url_cache_max_age_sec`.\nMaximum number of seconds the response to a signed URL request will\nbe considered fresh. After this time period,\nthe response will be revalidated before being served.\nWhen serving responses to signed URL requests,\nCloud CDN will internally behave as though\nall responses from this backend had a \"Cache-Control: public,\nmax-age=[TTL]\" header, regardless of any existing Cache-Control\nheader. The actual headers served in responses will not be altered."]
    pub fn set_signed_url_cache_max_age_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.signed_url_cache_max_age_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `bypass_cache_on_request_headers`.\n"]
    pub fn set_bypass_cache_on_request_headers(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.bypass_cache_on_request_headers = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.bypass_cache_on_request_headers = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `cache_key_policy`.\n"]
    pub fn set_cache_key_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendBucketCdnPolicyElCacheKeyPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.cache_key_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.cache_key_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `negative_caching_policy`.\n"]
    pub fn set_negative_caching_policy(
        mut self,
        v: impl Into<BlockAssignable<ComputeBackendBucketCdnPolicyElNegativeCachingPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.negative_caching_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.negative_caching_policy = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ComputeBackendBucketCdnPolicyEl {
    type O = BlockAssignable<ComputeBackendBucketCdnPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendBucketCdnPolicyEl {}
impl BuildComputeBackendBucketCdnPolicyEl {
    pub fn build(self) -> ComputeBackendBucketCdnPolicyEl {
        ComputeBackendBucketCdnPolicyEl {
            cache_mode: core::default::Default::default(),
            client_ttl: core::default::Default::default(),
            default_ttl: core::default::Default::default(),
            max_ttl: core::default::Default::default(),
            negative_caching: core::default::Default::default(),
            request_coalescing: core::default::Default::default(),
            serve_while_stale: core::default::Default::default(),
            signed_url_cache_max_age_sec: core::default::Default::default(),
            bypass_cache_on_request_headers: core::default::Default::default(),
            cache_key_policy: core::default::Default::default(),
            negative_caching_policy: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ComputeBackendBucketCdnPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendBucketCdnPolicyElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendBucketCdnPolicyElRef {
        ComputeBackendBucketCdnPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendBucketCdnPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `cache_mode` after provisioning.\nSpecifies the cache setting for all responses from this backend.\nThe possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: [\"USE_ORIGIN_HEADERS\", \"FORCE_CACHE_ALL\", \"CACHE_ALL_STATIC\"]"]
    pub fn cache_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cache_mode", self.base))
    }
    #[doc = "Get a reference to the value of field `client_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn client_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.client_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `default_ttl` after provisioning.\nSpecifies the default TTL for cached content served by this origin for responses\nthat do not have an existing valid TTL (max-age or s-max-age). When the 'cache_mode'\nis set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn default_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `max_ttl` after provisioning.\nSpecifies the maximum allowed TTL for cached content served by this origin. When the\n'cache_mode' is set to \"USE_ORIGIN_HEADERS\", you must omit this field."]
    pub fn max_ttl(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_ttl", self.base))
    }
    #[doc = "Get a reference to the value of field `negative_caching` after provisioning.\nNegative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects."]
    pub fn negative_caching(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.negative_caching", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `request_coalescing` after provisioning.\nIf true then Cloud CDN will combine multiple concurrent cache fill requests into a small number of requests to the origin."]
    pub fn request_coalescing(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.request_coalescing", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `serve_while_stale` after provisioning.\nServe existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache."]
    pub fn serve_while_stale(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.serve_while_stale", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `signed_url_cache_max_age_sec` after provisioning.\nMaximum number of seconds the response to a signed URL request will\nbe considered fresh. After this time period,\nthe response will be revalidated before being served.\nWhen serving responses to signed URL requests,\nCloud CDN will internally behave as though\nall responses from this backend had a \"Cache-Control: public,\nmax-age=[TTL]\" header, regardless of any existing Cache-Control\nheader. The actual headers served in responses will not be altered."]
    pub fn signed_url_cache_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.signed_url_cache_max_age_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `bypass_cache_on_request_headers` after provisioning.\n"]
    pub fn bypass_cache_on_request_headers(
        &self,
    ) -> ListRef<ComputeBackendBucketCdnPolicyElBypassCacheOnRequestHeadersElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.bypass_cache_on_request_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cache_key_policy` after provisioning.\n"]
    pub fn cache_key_policy(&self) -> ListRef<ComputeBackendBucketCdnPolicyElCacheKeyPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cache_key_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `negative_caching_policy` after provisioning.\n"]
    pub fn negative_caching_policy(
        &self,
    ) -> ListRef<ComputeBackendBucketCdnPolicyElNegativeCachingPolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.negative_caching_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendBucketParamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_manager_tags: Option<RecField<PrimField<String>>>,
}
impl ComputeBackendBucketParamsEl {
    #[doc = "Set the field `resource_manager_tags`.\nResource manager tags to be bound to the backend bucket. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn set_resource_manager_tags(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.resource_manager_tags = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendBucketParamsEl {
    type O = BlockAssignable<ComputeBackendBucketParamsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendBucketParamsEl {}
impl BuildComputeBackendBucketParamsEl {
    pub fn build(self) -> ComputeBackendBucketParamsEl {
        ComputeBackendBucketParamsEl {
            resource_manager_tags: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendBucketParamsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendBucketParamsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendBucketParamsElRef {
        ComputeBackendBucketParamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendBucketParamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `resource_manager_tags` after provisioning.\nResource manager tags to be bound to the backend bucket. Tag keys and values have the\nsame definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id},\nand values are in the format tagValues/456."]
    pub fn resource_manager_tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.resource_manager_tags", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ComputeBackendBucketTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ComputeBackendBucketTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.delete = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for ComputeBackendBucketTimeoutsEl {
    type O = BlockAssignable<ComputeBackendBucketTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildComputeBackendBucketTimeoutsEl {}
impl BuildComputeBackendBucketTimeoutsEl {
    pub fn build(self) -> ComputeBackendBucketTimeoutsEl {
        ComputeBackendBucketTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ComputeBackendBucketTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ComputeBackendBucketTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> ComputeBackendBucketTimeoutsElRef {
        ComputeBackendBucketTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ComputeBackendBucketTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct ComputeBackendBucketDynamic {
    cdn_policy: Option<DynamicBlock<ComputeBackendBucketCdnPolicyEl>>,
    params: Option<DynamicBlock<ComputeBackendBucketParamsEl>>,
}
