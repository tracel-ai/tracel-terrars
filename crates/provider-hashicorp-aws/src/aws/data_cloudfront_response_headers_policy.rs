use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataCloudfrontResponseHeadersPolicyData {
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
struct DataCloudfrontResponseHeadersPolicy_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataCloudfrontResponseHeadersPolicyData>,
}
#[derive(Clone)]
pub struct DataCloudfrontResponseHeadersPolicy(Rc<DataCloudfrontResponseHeadersPolicy_>);
impl DataCloudfrontResponseHeadersPolicy {
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
    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cors_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_headers_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remove_headers_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_headers_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_timing_headers_config", self.extract_ref()),
        )
    }
}
impl Referable for DataCloudfrontResponseHeadersPolicy {
    fn extract_ref(&self) -> String {
        format!(
            "data.{}.{}",
            self.0.extract_datasource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Datasource for DataCloudfrontResponseHeadersPolicy {}
impl ToListMappable for DataCloudfrontResponseHeadersPolicy {
    type O = ListRef<DataCloudfrontResponseHeadersPolicyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Datasource_ for DataCloudfrontResponseHeadersPolicy_ {
    fn extract_datasource_type(&self) -> String {
        "aws_cloudfront_response_headers_policy".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicy {
    pub tf_id: String,
}
impl BuildDataCloudfrontResponseHeadersPolicy {
    pub fn build(self, stack: &mut Stack) -> DataCloudfrontResponseHeadersPolicy {
        let out =
            DataCloudfrontResponseHeadersPolicy(Rc::new(DataCloudfrontResponseHeadersPolicy_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(DataCloudfrontResponseHeadersPolicyData {
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
pub struct DataCloudfrontResponseHeadersPolicyRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataCloudfrontResponseHeadersPolicyRef {
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
    #[doc = "Get a reference to the value of field `cors_config` after provisioning.\n"]
    pub fn cors_config(&self) -> ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.cors_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom_headers_config` after provisioning.\n"]
    pub fn custom_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom_headers_config", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `remove_headers_config` after provisioning.\n"]
    pub fn remove_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.remove_headers_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `security_headers_config` after provisioning.\n"]
    pub fn security_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.security_headers_config", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `server_timing_headers_config` after provisioning.\n"]
    pub fn server_timing_headers_config(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.server_timing_headers_config", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    type O =
        BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {}
impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    type O =
        BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {}
impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    type O =
        BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {}
impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<PrimField<String>>>,
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl
{
    type O = BlockAssignable<
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {}
impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_allow_credentials: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_allow_headers: Option<
        ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_allow_methods: Option<
        ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_allow_origins: Option<
        ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_expose_headers: Option<
        ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_max_age_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin_override: Option<PrimField<bool>>,
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigEl {
    #[doc = "Set the field `access_control_allow_credentials`.\n"]
    pub fn set_access_control_allow_credentials(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.access_control_allow_credentials = Some(v.into());
        self
    }
    #[doc = "Set the field `access_control_allow_headers`.\n"]
    pub fn set_access_control_allow_headers(
        mut self,
        v: impl Into<
            ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersEl>,
        >,
    ) -> Self {
        self.access_control_allow_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `access_control_allow_methods`.\n"]
    pub fn set_access_control_allow_methods(
        mut self,
        v: impl Into<
            ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsEl>,
        >,
    ) -> Self {
        self.access_control_allow_methods = Some(v.into());
        self
    }
    #[doc = "Set the field `access_control_allow_origins`.\n"]
    pub fn set_access_control_allow_origins(
        mut self,
        v: impl Into<
            ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsEl>,
        >,
    ) -> Self {
        self.access_control_allow_origins = Some(v.into());
        self
    }
    #[doc = "Set the field `access_control_expose_headers`.\n"]
    pub fn set_access_control_expose_headers(
        mut self,
        v: impl Into<
            ListField<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersEl>,
        >,
    ) -> Self {
        self.access_control_expose_headers = Some(v.into());
        self
    }
    #[doc = "Set the field `access_control_max_age_sec`.\n"]
    pub fn set_access_control_max_age_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.access_control_max_age_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `origin_override`.\n"]
    pub fn set_origin_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.origin_override = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyCorsConfigEl {
    type O = BlockAssignable<DataCloudfrontResponseHeadersPolicyCorsConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyCorsConfigEl {}
impl BuildDataCloudfrontResponseHeadersPolicyCorsConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCorsConfigEl {
        DataCloudfrontResponseHeadersPolicyCorsConfigEl {
            access_control_allow_credentials: core::default::Default::default(),
            access_control_allow_headers: core::default::Default::default(),
            access_control_allow_methods: core::default::Default::default(),
            access_control_allow_origins: core::default::Default::default(),
            access_control_expose_headers: core::default::Default::default(),
            access_control_max_age_sec: core::default::Default::default(),
            origin_override: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
        DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyCorsConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_control_allow_credentials` after provisioning.\n"]
    pub fn access_control_allow_credentials(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_control_allow_credentials", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_control_allow_headers` after provisioning.\n"]
    pub fn access_control_allow_headers(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowHeadersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_control_allow_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_control_allow_methods` after provisioning.\n"]
    pub fn access_control_allow_methods(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowMethodsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_control_allow_methods", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_control_allow_origins` after provisioning.\n"]
    pub fn access_control_allow_origins(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlAllowOriginsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_control_allow_origins", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_control_expose_headers` after provisioning.\n"]
    pub fn access_control_expose_headers(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicyCorsConfigElAccessControlExposeHeadersElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.access_control_expose_headers", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_control_max_age_sec` after provisioning.\n"]
    pub fn access_control_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_control_max_age_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `origin_override` after provisioning.\n"]
    pub fn origin_override(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.origin_override", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PrimField<String>>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}
impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header = Some(v.into());
        self
    }
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_ = Some(v.into());
        self
    }
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    type O = BlockAssignable<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {}
impl BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl {
            header: core::default::Default::default(),
            override_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef {
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
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
}
impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<SetField<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsEl>>,
    ) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    type O = BlockAssignable<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {}
impl BuildDataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
        DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> SetRef<DataCloudfrontResponseHeadersPolicyCustomHeadersConfigElItemsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<PrimField<String>>,
}
impl DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.header = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    type O = BlockAssignable<DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {}
impl BuildDataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
        DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl {
            header: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
        DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<SetField<DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl>>,
}
impl DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl Into<SetField<DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsEl>>,
    ) -> Self {
        self.items = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    type O = BlockAssignable<DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {}
impl BuildDataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
        DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigEl {
            items: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
        DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(
        &self,
    ) -> SetRef<DataCloudfrontResponseHeadersPolicyRemoveHeadersConfigElItemsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_security_policy: Option<PrimField<String>>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl
{
    type O = BlockAssignable<
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
}
impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl {
            content_security_policy: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.content_security_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_ = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl
{
    type O = BlockAssignable<
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {}
impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl {
            override_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    frame_option: Option<PrimField<String>>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    #[doc = "Set the field `frame_option`.\n"]
    pub fn set_frame_option(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.frame_option = Some(v.into());
        self
    }
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_ = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    type O =
        BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {}
impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl {
            frame_option: core::default::Default::default(),
            override_: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef {
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
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referrer_policy: Option<PrimField<String>>,
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_ = Some(v.into());
        self
    }
    #[doc = "Set the field `referrer_policy`.\n"]
    pub fn set_referrer_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.referrer_policy = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    type O =
        BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {}
impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl {
            override_: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.override", self.base))
    }
    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.referrer_policy", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_control_max_age_sec: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include_subdomains: Option<PrimField<bool>>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preload: Option<PrimField<bool>>,
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    #[doc = "Set the field `access_control_max_age_sec`.\n"]
    pub fn set_access_control_max_age_sec(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.access_control_max_age_sec = Some(v.into());
        self
    }
    #[doc = "Set the field `include_subdomains`.\n"]
    pub fn set_include_subdomains(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_subdomains = Some(v.into());
        self
    }
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_ = Some(v.into());
        self
    }
    #[doc = "Set the field `preload`.\n"]
    pub fn set_preload(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.preload = Some(v.into());
        self
    }
}
impl ToListMappable
    for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl
{
    type O = BlockAssignable<
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl
{}
impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl {
            access_control_max_age_sec: core::default::Default::default(),
            include_subdomains: core::default::Default::default(),
            override_: core::default::Default::default(),
            preload: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef
    {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_control_max_age_sec` after provisioning.\n"]
    pub fn access_control_max_age_sec(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_control_max_age_sec", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `include_subdomains` after provisioning.\n"]
    pub fn include_subdomains(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.include_subdomains", self.base),
        )
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
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode_block: Option<PrimField<bool>>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    override_: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    report_uri: Option<PrimField<String>>,
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    #[doc = "Set the field `mode_block`.\n"]
    pub fn set_mode_block(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.mode_block = Some(v.into());
        self
    }
    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.override_ = Some(v.into());
        self
    }
    #[doc = "Set the field `protection`.\n"]
    pub fn set_protection(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.protection = Some(v.into());
        self
    }
    #[doc = "Set the field `report_uri`.\n"]
    pub fn set_report_uri(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.report_uri = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    type O =
        BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {}
impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
    pub fn build(
        self,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl {
            mode_block: core::default::Default::default(),
            override_: core::default::Default::default(),
            protection: core::default::Default::default(),
            report_uri: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef {
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
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_security_policy: Option<
        ListField<
            DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type_options: Option<
        ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    frame_options:
        Option<ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    referrer_policy: Option<
        ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict_transport_security: Option<
        ListField<
            DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl,
        >,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    xss_protection: Option<
        ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>,
    >,
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    #[doc = "Set the field `content_security_policy`.\n"]
    pub fn set_content_security_policy(
        mut self,
        v: impl Into<
            ListField<
                DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyEl,
            >,
        >,
    ) -> Self {
        self.content_security_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `content_type_options`.\n"]
    pub fn set_content_type_options(
        mut self,
        v: impl Into<
            ListField<
                DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsEl,
            >,
        >,
    ) -> Self {
        self.content_type_options = Some(v.into());
        self
    }
    #[doc = "Set the field `frame_options`.\n"]
    pub fn set_frame_options(
        mut self,
        v: impl Into<
            ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsEl>,
        >,
    ) -> Self {
        self.frame_options = Some(v.into());
        self
    }
    #[doc = "Set the field `referrer_policy`.\n"]
    pub fn set_referrer_policy(
        mut self,
        v: impl Into<
            ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyEl>,
        >,
    ) -> Self {
        self.referrer_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `strict_transport_security`.\n"]
    pub fn set_strict_transport_security(
        mut self,
        v: impl Into<
            ListField<
                DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityEl,
            >,
        >,
    ) -> Self {
        self.strict_transport_security = Some(v.into());
        self
    }
    #[doc = "Set the field `xss_protection`.\n"]
    pub fn set_xss_protection(
        mut self,
        v: impl Into<
            ListField<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionEl>,
        >,
    ) -> Self {
        self.xss_protection = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    type O = BlockAssignable<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {}
impl BuildDataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigEl {
            content_security_policy: core::default::Default::default(),
            content_type_options: core::default::Default::default(),
            frame_options: core::default::Default::default(),
            referrer_policy: core::default::Default::default(),
            strict_transport_security: core::default::Default::default(),
            xss_protection: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `content_security_policy` after provisioning.\n"]
    pub fn content_security_policy(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentSecurityPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_security_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `content_type_options` after provisioning.\n"]
    pub fn content_type_options(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElContentTypeOptionsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.content_type_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `frame_options` after provisioning.\n"]
    pub fn frame_options(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElFrameOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.frame_options", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `referrer_policy` after provisioning.\n"]
    pub fn referrer_policy(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElReferrerPolicyElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.referrer_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `strict_transport_security` after provisioning.\n"]
    pub fn strict_transport_security(
        &self,
    ) -> ListRef<
        DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElStrictTransportSecurityElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.strict_transport_security", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `xss_protection` after provisioning.\n"]
    pub fn xss_protection(
        &self,
    ) -> ListRef<DataCloudfrontResponseHeadersPolicySecurityHeadersConfigElXssProtectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.xss_protection", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sampling_rate: Option<PrimField<f64>>,
}
impl DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
    #[doc = "Set the field `sampling_rate`.\n"]
    pub fn set_sampling_rate(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.sampling_rate = Some(v.into());
        self
    }
}
impl ToListMappable for DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    type O = BlockAssignable<DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {}
impl BuildDataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
    pub fn build(self) -> DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
        DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigEl {
            enabled: core::default::Default::default(),
            sampling_rate: core::default::Default::default(),
        }
    }
}
pub struct DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
        DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataCloudfrontResponseHeadersPolicyServerTimingHeadersConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
    #[doc = "Get a reference to the value of field `sampling_rate` after provisioning.\n"]
    pub fn sampling_rate(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sampling_rate", self.base),
        )
    }
}
