use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontResponseHeadersPolicyData {
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
    cors_config: Option<Vec<CloudfrontResponseHeadersPolicyCorsConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_headers_config: Option<Vec<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remove_headers_config: Option<Vec<CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_headers_config: Option<Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_timing_headers_config: Option<Vec<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>>,
    dynamic: CloudfrontResponseHeadersPolicyDynamic,
}

struct CloudfrontResponseHeadersPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CloudfrontResponseHeadersPolicyData>,
}

#[derive(Clone)]
pub struct CloudfrontResponseHeadersPolicy(Rc<CloudfrontResponseHeadersPolicy_>);

impl CloudfrontResponseHeadersPolicy {
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
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
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
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

    #[doc = "Set the field `cors_config`.\n"]
    pub fn set_cors_config(self, v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_headers_config`.\n"]
    pub fn set_custom_headers_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_headers_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_headers_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `remove_headers_config`.\n"]
    pub fn set_remove_headers_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().remove_headers_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.remove_headers_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `security_headers_config`.\n"]
    pub fn set_security_headers_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().security_headers_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.security_headers_config = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `server_timing_headers_config`.\n"]
    pub fn set_server_timing_headers_config(
        self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_timing_headers_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_timing_headers_config = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyCorsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remove_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

impl Referable for CloudfrontResponseHeadersPolicy {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for CloudfrontResponseHeadersPolicy { }

impl ToListMappable for CloudfrontResponseHeadersPolicy {
    type O = ListRef<CloudfrontResponseHeadersPolicyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CloudfrontResponseHeadersPolicy_ {
    fn extract_resource_type(&self) -> String {
        "aws_cloudfront_response_headers_policy".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontResponseHeadersPolicy {
    pub tf_id: String,
    #[doc = ""]
    pub name: PrimField<String>,
}

impl BuildCloudfrontResponseHeadersPolicy {
    pub fn build(self, stack: &mut Stack) -> CloudfrontResponseHeadersPolicy {
        let out = CloudfrontResponseHeadersPolicy(Rc::new(CloudfrontResponseHeadersPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CloudfrontResponseHeadersPolicyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                comment: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                cors_config: core::default::Default::default(),
                custom_headers_config: core::default::Default::default(),
                remove_headers_config: core::default::Default::default(),
                security_headers_config: core::default::Default::default(),
                server_timing_headers_config: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontResponseHeadersPolicyRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl CloudfrontResponseHeadersPolicyRef {
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
        PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyCorsConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.custom_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.remove_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.security_headers_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(&self) -> ListRef<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_timing_headers_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
            items: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicyCorsConfigElDynamic {
    access_control_allow_headers: Option<
        DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>,
    >,
    access_control_allow_methods: Option<
        DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>,
    >,
    access_control_allow_origins: Option<
        DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>,
    >,
    access_control_expose_headers: Option<
        DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCorsConfigEl {
    access_control_allow_credentials: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_max_age_sec: Option<PrimField<f64>>,
    origin_override: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_allow_headers: Option<Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_allow_methods: Option<Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_allow_origins: Option<Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_expose_headers: Option<Vec<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>>,
    dynamic: CloudfrontResponseHeadersPolicyCorsConfigElDynamic,
}

impl CloudfrontResponseHeadersPolicyCorsConfigEl {
    #[doc = "Set the field `access_control_max_age_sec`.\n"]
    pub fn set_access_control_max_age_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.access_control_max_age_sec = Some(v.into());
        self
    }

    #[doc = "Set the field `access_control_allow_headers`.\n"]
    pub fn set_access_control_allow_headers(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_control_allow_headers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_allow_headers = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `access_control_allow_methods`.\n"]
    pub fn set_access_control_allow_methods(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_control_allow_methods = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_allow_methods = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `access_control_allow_origins`.\n"]
    pub fn set_access_control_allow_origins(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_control_allow_origins = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_allow_origins = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `access_control_expose_headers`.\n"]
    pub fn set_access_control_expose_headers(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.access_control_expose_headers = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.access_control_expose_headers = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicyCorsConfigEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyCorsConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCorsConfigEl {
    #[doc = ""]
    pub access_control_allow_credentials: PrimField<bool>,
    #[doc = ""]
    pub origin_override: PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicyCorsConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCorsConfigEl {
        CloudfrontResponseHeadersPolicyCorsConfigEl {
            access_control_allow_credentials: self.access_control_allow_credentials,
            access_control_max_age_sec: core::default::Default::default(),
            origin_override: self.origin_override,
            access_control_allow_headers: core::default::Default::default(),
            access_control_allow_methods: core::default::Default::default(),
            access_control_allow_origins: core::default::Default::default(),
            access_control_expose_headers: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCorsConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyCorsConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontResponseHeadersPolicyCorsConfigElRef {
        CloudfrontResponseHeadersPolicyCorsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCorsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_control_allow_credentials` after provisioning.\n"]
    pub fn access_control_allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_control_allow_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_max_age_sec` after provisioning.\n"]
    pub fn access_control_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_control_max_age_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `origin_override` after provisioning.\n"]
    pub fn origin_override(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.origin_override", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_headers` after provisioning.\n"]
    pub fn access_control_allow_headers(
        &self,
    ) -> ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_control_allow_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_methods` after provisioning.\n"]
    pub fn access_control_allow_methods(
        &self,
    ) -> ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_control_allow_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_allow_origins` after provisioning.\n"]
    pub fn access_control_allow_origins(
        &self,
    ) -> ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_control_allow_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `access_control_expose_headers` after provisioning.\n"]
    pub fn access_control_expose_headers(
        &self,
    ) -> ListRef<CloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef> {
        ListRef::new(self.shared().clone(), format!("{}.access_control_expose_headers", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    header: PrimField<String>,
    #[serde(rename = "override")]
    override_: PrimField<bool>,
    value: PrimField<String>,
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl { }

impl ToListMappable for CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    #[doc = ""]
    pub header: PrimField<String>,
    #[doc = ""]
    pub override_: PrimField<bool>,
    #[doc = ""]
    pub value: PrimField<String>,
}

impl BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
            header: self.header,
            override_: self.override_,
            value: self.value,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElDynamic {
    items: Option<DynamicBlock<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
    dynamic: CloudfrontResponseHeadersPolicyCustomHeadersConfigElDynamic,
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.items = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.items = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {}

impl BuildCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
        CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    header: PrimField<String>,
}

impl CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl { }

impl ToListMappable for CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    #[doc = ""]
    pub header: PrimField<String>,
}

impl BuildCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
        CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl { header: self.header }
    }
}

pub struct CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
        CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicyRemoveHeadersConfigElDynamic {
    items: Option<DynamicBlock<CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl>>,
    dynamic: CloudfrontResponseHeadersPolicyRemoveHeadersConfigElDynamic,
}

impl CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.items = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.items = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {}

impl BuildCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
        CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
            items: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
        CloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    content_security_policy: PrimField<String>,
    #[serde(rename = "override")]
    override_: PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl { }

impl ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    #[doc = ""]
    pub content_security_policy: PrimField<String>,
    #[doc = ""]
    pub override_: PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
            content_security_policy: self.content_security_policy,
            override_: self.override_,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_security_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[serde(rename = "override")]
    override_: PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl { }

impl ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[doc = ""]
    pub override_: PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl { override_: self.override_ }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    frame_option: PrimField<String>,
    #[serde(rename = "override")]
    override_: PrimField<bool>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl { }

impl ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    #[doc = ""]
    pub frame_option: PrimField<String>,
    #[doc = ""]
    pub override_: PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
            frame_option: self.frame_option,
            override_: self.override_,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `frame_option` after provisioning.\n"]
    pub fn frame_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.frame_option", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    #[serde(rename = "override")]
    override_: PrimField<bool>,
    referrer_policy: PrimField<String>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl { }

impl ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    #[doc = ""]
    pub override_: PrimField<bool>,
    #[doc = ""]
    pub referrer_policy: PrimField<String>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
            override_: self.override_,
            referrer_policy: self.referrer_policy,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.referrer_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    access_control_max_age_sec: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_subdomains: Option<PrimField<bool>>,
    #[serde(rename = "override")]
    override_: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preload: Option<PrimField<bool>>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    #[doc = "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_subdomains = Some(v.into());
        self
    }

    #[doc = "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preload = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    #[doc = ""]
    pub access_control_max_age_sec: PrimField<f64>,
    #[doc = ""]
    pub override_: PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
            access_control_max_age_sec: self.access_control_max_age_sec,
            include_subdomains: core::default::Default::default(),
            override_: self.override_,
            preload: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_control_max_age_sec` after provisioning.\n"]
    pub fn access_control_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.access_control_max_age_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_subdomains", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `preload` after provisioning.\n"]
    pub fn preload(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.preload", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode_block: Option<PrimField<bool>>,
    #[serde(rename = "override")]
    override_: PrimField<bool>,
    protection: PrimField<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_uri: Option<PrimField<String>>,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[doc = "Set the field `mode_block`.\n"]
    pub fn set_mode_block(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mode_block = Some(v.into());
        self
    }

    #[doc = "Set the field `report_uri`.\n"]
    pub fn set_report_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.report_uri = Some(v.into());
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[doc = ""]
    pub override_: PrimField<bool>,
    #[doc = ""]
    pub protection: PrimField<bool>,
}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
            mode_block: core::default::Default::default(),
            override_: self.override_,
            protection: self.protection,
            report_uri: core::default::Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mode_block` after provisioning.\n"]
    pub fn mode_block(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode_block", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `protection` after provisioning.\n"]
    pub fn protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.protection", self.base))
    }

    #[doc = "Get a reference to the value of field `report_uri` after provisioning.\n"]
    pub fn report_uri(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.report_uri", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElDynamic {
    content_security_policy: Option<
        DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>,
    >,
    content_type_options: Option<
        DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>,
    >,
    frame_options: Option<DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>>,
    referrer_policy: Option<DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>>,
    strict_transport_security: Option<
        DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>,
    >,
    xss_protection: Option<DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>>,
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_security_policy: Option<Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type_options: Option<Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frame_options: Option<Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referrer_policy: Option<Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_transport_security: Option<
        Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    xss_protection: Option<Vec<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>>,
    dynamic: CloudfrontResponseHeadersPolicySecurityHeadersConfigElDynamic,
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.content_security_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.content_security_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `content_type_options`.\n"]
    pub fn set_content_type_options(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.content_type_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.content_type_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `frame_options`.\n"]
    pub fn set_frame_options(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.frame_options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.frame_options = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `referrer_policy`.\n"]
    pub fn set_referrer_policy(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.referrer_policy = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.referrer_policy = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `strict_transport_security`.\n"]
    pub fn set_strict_transport_security(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.strict_transport_security = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.strict_transport_security = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xss_protection`.\n"]
    pub fn set_xss_protection(
        mut self,
        v: impl Into<BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.xss_protection = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.xss_protection = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {}

impl BuildCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
            content_security_policy: core::default::Default::default(),
            content_type_options: core::default::Default::default(),
            frame_options: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
            strict_transport_security: core::default::Default::default(),
            xss_protection: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
        CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(
        &self,
    ) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_security_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type_options` after provisioning.\n"]
    pub fn content_type_options(
        &self,
    ) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.content_type_options", self.base))
    }

    #[doc = "Get a reference to the value of field `frame_options` after provisioning.\n"]
    pub fn frame_options(&self) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.frame_options", self.base))
    }

    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(&self) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.referrer_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `strict_transport_security` after provisioning.\n"]
    pub fn strict_transport_security(
        &self,
    ) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.strict_transport_security", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_protection` after provisioning.\n"]
    pub fn xss_protection(&self) -> ListRef<CloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.xss_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    enabled: PrimField<bool>,
    sampling_rate: PrimField<f64>,
}

impl CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl { }

impl ToListMappable for CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    type O = BlockAssignable<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    #[doc = ""]
    pub enabled: PrimField<bool>,
    #[doc = ""]
    pub sampling_rate: PrimField<f64>,
}

impl BuildCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    pub fn build(self) -> CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
        CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
            enabled: self.enabled,
            sampling_rate: self.sampling_rate,
        }
    }
}

pub struct CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    fn new(shared: StackShared, base: String) -> CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
        CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `sampling_rate` after provisioning.\n"]
    pub fn sampling_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sampling_rate", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontResponseHeadersPolicyDynamic {
    cors_config: Option<DynamicBlock<CloudfrontResponseHeadersPolicyCorsConfigEl>>,
    custom_headers_config: Option<DynamicBlock<CloudfrontResponseHeadersPolicyCustomHeadersConfigEl>>,
    remove_headers_config: Option<DynamicBlock<CloudfrontResponseHeadersPolicyRemoveHeadersConfigEl>>,
    security_headers_config: Option<DynamicBlock<CloudfrontResponseHeadersPolicySecurityHeadersConfigEl>>,
    server_timing_headers_config: Option<DynamicBlock<CloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>>,
}
