use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct RecaptchaEnterpriseKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deletion_policy: Option<PrimField<String>>,
    display_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    android_settings: Option<Vec<RecaptchaEnterpriseKeyAndroidSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ios_settings: Option<Vec<RecaptchaEnterpriseKeyIosSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    testing_options: Option<Vec<RecaptchaEnterpriseKeyTestingOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<RecaptchaEnterpriseKeyTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    waf_settings: Option<Vec<RecaptchaEnterpriseKeyWafSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_settings: Option<Vec<RecaptchaEnterpriseKeyWebSettingsEl>>,
    dynamic: RecaptchaEnterpriseKeyDynamic,
}
struct RecaptchaEnterpriseKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<RecaptchaEnterpriseKeyData>,
}
#[derive(Clone)]
pub struct RecaptchaEnterpriseKey(Rc<RecaptchaEnterpriseKey_>);
impl RecaptchaEnterpriseKey {
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
    #[doc = "Set the field `deletion_policy`.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn set_deletion_policy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().deletion_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `labels`.\nSee [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn set_labels(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().labels = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\nThe project for the resource"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `android_settings`.\n"]
    pub fn set_android_settings(
        self,
        v: impl Into<BlockAssignable<RecaptchaEnterpriseKeyAndroidSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().android_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.android_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `ios_settings`.\n"]
    pub fn set_ios_settings(
        self,
        v: impl Into<BlockAssignable<RecaptchaEnterpriseKeyIosSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ios_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ios_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `testing_options`.\n"]
    pub fn set_testing_options(
        self,
        v: impl Into<BlockAssignable<RecaptchaEnterpriseKeyTestingOptionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().testing_options = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.testing_options = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<RecaptchaEnterpriseKeyTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `waf_settings`.\n"]
    pub fn set_waf_settings(
        self,
        v: impl Into<BlockAssignable<RecaptchaEnterpriseKeyWafSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().waf_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.waf_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `web_settings`.\n"]
    pub fn set_web_settings(
        self,
        v: impl Into<BlockAssignable<RecaptchaEnterpriseKeyWebSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().web_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.web_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp corresponding to the creation of this Key."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this key. Modifiable by user."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSee [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource id for the Key, which is the same as the Site Key itself."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `android_settings` after provisioning.\n"]
    pub fn android_settings(&self) -> ListRef<RecaptchaEnterpriseKeyAndroidSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.android_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ios_settings` after provisioning.\n"]
    pub fn ios_settings(&self) -> ListRef<RecaptchaEnterpriseKeyIosSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ios_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `testing_options` after provisioning.\n"]
    pub fn testing_options(&self) -> ListRef<RecaptchaEnterpriseKeyTestingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.testing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecaptchaEnterpriseKeyTimeoutsElRef {
        RecaptchaEnterpriseKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `waf_settings` after provisioning.\n"]
    pub fn waf_settings(&self) -> ListRef<RecaptchaEnterpriseKeyWafSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.waf_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_settings` after provisioning.\n"]
    pub fn web_settings(&self) -> ListRef<RecaptchaEnterpriseKeyWebSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_settings", self.extract_ref()),
        )
    }
}
impl Referable for RecaptchaEnterpriseKey {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for RecaptchaEnterpriseKey {}
impl ToListMappable for RecaptchaEnterpriseKey {
    type O = ListRef<RecaptchaEnterpriseKeyRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for RecaptchaEnterpriseKey_ {
    fn extract_resource_type(&self) -> String {
        "google_recaptcha_enterprise_key".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildRecaptchaEnterpriseKey {
    pub tf_id: String,
    #[doc = "Human-readable display name of this key. Modifiable by user."]
    pub display_name: PrimField<String>,
}
impl BuildRecaptchaEnterpriseKey {
    pub fn build(self, stack: &mut Stack) -> RecaptchaEnterpriseKey {
        let out = RecaptchaEnterpriseKey(Rc::new(RecaptchaEnterpriseKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(RecaptchaEnterpriseKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                display_name: self.display_name,
                id: core::default::Default::default(),
                labels: core::default::Default::default(),
                project: core::default::Default::default(),
                android_settings: core::default::Default::default(),
                ios_settings: core::default::Default::default(),
                testing_options: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                waf_settings: core::default::Default::default(),
                web_settings: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct RecaptchaEnterpriseKeyRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl RecaptchaEnterpriseKeyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nThe timestamp corresponding to the creation of this Key."]
    pub fn create_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.create_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nHuman-readable display name of this key. Modifiable by user."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `effective_labels` after provisioning.\nAll of labels (key/value pairs) present on the resource in GCP, including the labels configured through Terraform, other clients and services."]
    pub fn effective_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.effective_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `labels` after provisioning.\nSee [Creating and managing labels](https://cloud.google.com/recaptcha-enterprise/docs/labels).\n\n**Note**: This field is non-authoritative, and will only manage the labels present in your configuration.\nPlease refer to the field `effective_labels` for all of the labels present on the resource."]
    pub fn labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nThe resource id for the Key, which is the same as the Site Key itself."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\nThe project for the resource"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `terraform_labels` after provisioning.\nThe combination of labels configured directly on the resource and default labels configured on the provider."]
    pub fn terraform_labels(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.terraform_labels", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `android_settings` after provisioning.\n"]
    pub fn android_settings(&self) -> ListRef<RecaptchaEnterpriseKeyAndroidSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.android_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ios_settings` after provisioning.\n"]
    pub fn ios_settings(&self) -> ListRef<RecaptchaEnterpriseKeyIosSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ios_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `testing_options` after provisioning.\n"]
    pub fn testing_options(&self) -> ListRef<RecaptchaEnterpriseKeyTestingOptionsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.testing_options", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RecaptchaEnterpriseKeyTimeoutsElRef {
        RecaptchaEnterpriseKeyTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `waf_settings` after provisioning.\n"]
    pub fn waf_settings(&self) -> ListRef<RecaptchaEnterpriseKeyWafSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.waf_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `web_settings` after provisioning.\n"]
    pub fn web_settings(&self) -> ListRef<RecaptchaEnterpriseKeyWebSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.web_settings", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyAndroidSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_package_names: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_package_names: Option<ListField<PrimField<String>>>,
}
impl RecaptchaEnterpriseKeyAndroidSettingsEl {
    #[doc = "Set the field `allow_all_package_names`.\nIf set to true, it means allowed_package_names will not be enforced."]
    pub fn set_allow_all_package_names(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_package_names = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_package_names`.\nAndroid package names of apps allowed to use the key. Example: 'com.companyname.appname'"]
    pub fn set_allowed_package_names(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_package_names = Some(v.into());
        self
    }
}
impl ToListMappable for RecaptchaEnterpriseKeyAndroidSettingsEl {
    type O = BlockAssignable<RecaptchaEnterpriseKeyAndroidSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyAndroidSettingsEl {}
impl BuildRecaptchaEnterpriseKeyAndroidSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyAndroidSettingsEl {
        RecaptchaEnterpriseKeyAndroidSettingsEl {
            allow_all_package_names: core::default::Default::default(),
            allowed_package_names: core::default::Default::default(),
        }
    }
}
pub struct RecaptchaEnterpriseKeyAndroidSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyAndroidSettingsElRef {
    fn new(shared: StackShared, base: String) -> RecaptchaEnterpriseKeyAndroidSettingsElRef {
        RecaptchaEnterpriseKeyAndroidSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyAndroidSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all_package_names` after provisioning.\nIf set to true, it means allowed_package_names will not be enforced."]
    pub fn allow_all_package_names(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_all_package_names", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_package_names` after provisioning.\nAndroid package names of apps allowed to use the key. Example: 'com.companyname.appname'"]
    pub fn allowed_package_names(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_package_names", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyIosSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_bundle_ids: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_bundle_ids: Option<ListField<PrimField<String>>>,
}
impl RecaptchaEnterpriseKeyIosSettingsEl {
    #[doc = "Set the field `allow_all_bundle_ids`.\nIf set to true, it means allowed_bundle_ids will not be enforced."]
    pub fn set_allow_all_bundle_ids(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_bundle_ids = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_bundle_ids`.\niOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'"]
    pub fn set_allowed_bundle_ids(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_bundle_ids = Some(v.into());
        self
    }
}
impl ToListMappable for RecaptchaEnterpriseKeyIosSettingsEl {
    type O = BlockAssignable<RecaptchaEnterpriseKeyIosSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyIosSettingsEl {}
impl BuildRecaptchaEnterpriseKeyIosSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyIosSettingsEl {
        RecaptchaEnterpriseKeyIosSettingsEl {
            allow_all_bundle_ids: core::default::Default::default(),
            allowed_bundle_ids: core::default::Default::default(),
        }
    }
}
pub struct RecaptchaEnterpriseKeyIosSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyIosSettingsElRef {
    fn new(shared: StackShared, base: String) -> RecaptchaEnterpriseKeyIosSettingsElRef {
        RecaptchaEnterpriseKeyIosSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyIosSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all_bundle_ids` after provisioning.\nIf set to true, it means allowed_bundle_ids will not be enforced."]
    pub fn allow_all_bundle_ids(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_all_bundle_ids", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_bundle_ids` after provisioning.\niOS bundle ids of apps allowed to use the key. Example: 'com.companyname.productname.appname'"]
    pub fn allowed_bundle_ids(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_bundle_ids", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyTestingOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    testing_challenge: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    testing_score: Option<PrimField<f64>>,
}
impl RecaptchaEnterpriseKeyTestingOptionsEl {
    #[doc = "Set the field `testing_challenge`.\nFor challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if UNSOLVABLE_CHALLENGE. Possible values: TESTING_CHALLENGE_UNSPECIFIED, NOCAPTCHA, UNSOLVABLE_CHALLENGE"]
    pub fn set_testing_challenge(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.testing_challenge = Some(v.into());
        self
    }
    #[doc = "Set the field `testing_score`.\nAll assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive."]
    pub fn set_testing_score(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.testing_score = Some(v.into());
        self
    }
}
impl ToListMappable for RecaptchaEnterpriseKeyTestingOptionsEl {
    type O = BlockAssignable<RecaptchaEnterpriseKeyTestingOptionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyTestingOptionsEl {}
impl BuildRecaptchaEnterpriseKeyTestingOptionsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyTestingOptionsEl {
        RecaptchaEnterpriseKeyTestingOptionsEl {
            testing_challenge: core::default::Default::default(),
            testing_score: core::default::Default::default(),
        }
    }
}
pub struct RecaptchaEnterpriseKeyTestingOptionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyTestingOptionsElRef {
    fn new(shared: StackShared, base: String) -> RecaptchaEnterpriseKeyTestingOptionsElRef {
        RecaptchaEnterpriseKeyTestingOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyTestingOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `testing_challenge` after provisioning.\nFor challenge-based keys only (CHECKBOX, INVISIBLE), all challenge requests for this site will return nocaptcha if NOCAPTCHA, or an unsolvable challenge if UNSOLVABLE_CHALLENGE. Possible values: TESTING_CHALLENGE_UNSPECIFIED, NOCAPTCHA, UNSOLVABLE_CHALLENGE"]
    pub fn testing_challenge(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.testing_challenge", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `testing_score` after provisioning.\nAll assessments for this Key will return this score. Must be between 0 (likely not legitimate) and 1 (likely legitimate) inclusive."]
    pub fn testing_score(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.testing_score", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl RecaptchaEnterpriseKeyTimeoutsEl {
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
impl ToListMappable for RecaptchaEnterpriseKeyTimeoutsEl {
    type O = BlockAssignable<RecaptchaEnterpriseKeyTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyTimeoutsEl {}
impl BuildRecaptchaEnterpriseKeyTimeoutsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyTimeoutsEl {
        RecaptchaEnterpriseKeyTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct RecaptchaEnterpriseKeyTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> RecaptchaEnterpriseKeyTimeoutsElRef {
        RecaptchaEnterpriseKeyTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyTimeoutsElRef {
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
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyWafSettingsEl {
    waf_feature: PrimField<String>,
    waf_service: PrimField<String>,
}
impl RecaptchaEnterpriseKeyWafSettingsEl {}
impl ToListMappable for RecaptchaEnterpriseKeyWafSettingsEl {
    type O = BlockAssignable<RecaptchaEnterpriseKeyWafSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyWafSettingsEl {
    #[doc = "Supported WAF features. For more information, see https://cloud.google.com/recaptcha-enterprise/docs/usecase#comparison_of_features. Possible values: CHALLENGE_PAGE, SESSION_TOKEN, ACTION_TOKEN, EXPRESS"]
    pub waf_feature: PrimField<String>,
    #[doc = "The WAF service that uses this key. Possible values: CA, FASTLY"]
    pub waf_service: PrimField<String>,
}
impl BuildRecaptchaEnterpriseKeyWafSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyWafSettingsEl {
        RecaptchaEnterpriseKeyWafSettingsEl {
            waf_feature: self.waf_feature,
            waf_service: self.waf_service,
        }
    }
}
pub struct RecaptchaEnterpriseKeyWafSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyWafSettingsElRef {
    fn new(shared: StackShared, base: String) -> RecaptchaEnterpriseKeyWafSettingsElRef {
        RecaptchaEnterpriseKeyWafSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyWafSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `waf_feature` after provisioning.\nSupported WAF features. For more information, see https://cloud.google.com/recaptcha-enterprise/docs/usecase#comparison_of_features. Possible values: CHALLENGE_PAGE, SESSION_TOKEN, ACTION_TOKEN, EXPRESS"]
    pub fn waf_feature(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waf_feature", self.base))
    }
    #[doc = "Get a reference to the value of field `waf_service` after provisioning.\nThe WAF service that uses this key. Possible values: CA, FASTLY"]
    pub fn waf_service(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.waf_service", self.base))
    }
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl {
    action: PrimField<String>,
    score_threshold: PrimField<f64>,
}
impl RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl {}
impl ToListMappable for RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl {
    type O =
        BlockAssignable<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl {
    #[doc = "The action name."]
    pub action: PrimField<String>,
    #[doc = "A challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub score_threshold: PrimField<f64>,
}
impl BuildRecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl {
        RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl {
            action: self.action,
            score_threshold: self.score_threshold,
        }
    }
}
pub struct RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsElRef {
        RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `action` after provisioning.\nThe action name."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
    #[doc = "Get a reference to the value of field `score_threshold` after provisioning.\nA challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub fn score_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.score_threshold", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl {
    score_threshold: PrimField<f64>,
}
impl RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl {}
impl ToListMappable for RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl {
    type O =
        BlockAssignable<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl {
    #[doc = "A challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub score_threshold: PrimField<f64>,
}
impl BuildRecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl {
        RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl {
            score_threshold: self.score_threshold,
        }
    }
}
pub struct RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsElRef {
        RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `score_threshold` after provisioning.\nA challenge is triggered if the end-user score is below that threshold. Value must be between 0 and 1 (inclusive)."]
    pub fn score_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.score_threshold", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDynamic {
    action_settings: Option<
        DynamicBlock<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl>,
    >,
    default_settings: Option<
        DynamicBlock<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl>,
    >,
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action_settings:
        Option<Vec<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_settings:
        Option<Vec<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl>>,
    dynamic: RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDynamic,
}
impl RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl {
    #[doc = "Set the field `action_settings`.\n"]
    pub fn set_action_settings(
        mut self,
        v: impl Into<
            BlockAssignable<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElActionSettingsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `default_settings`.\n"]
    pub fn set_default_settings(
        mut self,
        v: impl Into<
            BlockAssignable<
                RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl {
    type O = BlockAssignable<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl {}
impl BuildRecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl {
        RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl {
            action_settings: core::default::Default::default(),
            default_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElRef {
        RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `default_settings` after provisioning.\n"]
    pub fn default_settings(
        &self,
    ) -> ListRef<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElDefaultSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.default_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct RecaptchaEnterpriseKeyWebSettingsElDynamic {
    challenge_settings:
        Option<DynamicBlock<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl>>,
}
#[derive(Serialize)]
pub struct RecaptchaEnterpriseKeyWebSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_all_domains: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_amp_traffic: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_domains: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    challenge_security_preference: Option<PrimField<String>>,
    integration_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    challenge_settings: Option<Vec<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl>>,
    dynamic: RecaptchaEnterpriseKeyWebSettingsElDynamic,
}
impl RecaptchaEnterpriseKeyWebSettingsEl {
    #[doc = "Set the field `allow_all_domains`.\nIf set to true, it means allowed_domains will not be enforced."]
    pub fn set_allow_all_domains(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_all_domains = Some(v.into());
        self
    }
    #[doc = "Set the field `allow_amp_traffic`.\nIf set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type."]
    pub fn set_allow_amp_traffic(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.allow_amp_traffic = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_domains`.\nDomains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'"]
    pub fn set_allowed_domains(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_domains = Some(v.into());
        self
    }
    #[doc = "Set the field `challenge_security_preference`.\nSettings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE. Possible values: CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED, USABILITY, BALANCE, SECURITY"]
    pub fn set_challenge_security_preference(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.challenge_security_preference = Some(v.into());
        self
    }
    #[doc = "Set the field `challenge_settings`.\n"]
    pub fn set_challenge_settings(
        mut self,
        v: impl Into<BlockAssignable<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.challenge_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge_settings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for RecaptchaEnterpriseKeyWebSettingsEl {
    type O = BlockAssignable<RecaptchaEnterpriseKeyWebSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildRecaptchaEnterpriseKeyWebSettingsEl {
    #[doc = "Required. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE, POLICY_BASED_CHALLENGE"]
    pub integration_type: PrimField<String>,
}
impl BuildRecaptchaEnterpriseKeyWebSettingsEl {
    pub fn build(self) -> RecaptchaEnterpriseKeyWebSettingsEl {
        RecaptchaEnterpriseKeyWebSettingsEl {
            allow_all_domains: core::default::Default::default(),
            allow_amp_traffic: core::default::Default::default(),
            allowed_domains: core::default::Default::default(),
            challenge_security_preference: core::default::Default::default(),
            integration_type: self.integration_type,
            challenge_settings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct RecaptchaEnterpriseKeyWebSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for RecaptchaEnterpriseKeyWebSettingsElRef {
    fn new(shared: StackShared, base: String) -> RecaptchaEnterpriseKeyWebSettingsElRef {
        RecaptchaEnterpriseKeyWebSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl RecaptchaEnterpriseKeyWebSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allow_all_domains` after provisioning.\nIf set to true, it means allowed_domains will not be enforced."]
    pub fn allow_all_domains(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_all_domains", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allow_amp_traffic` after provisioning.\nIf set to true, the key can be used on AMP (Accelerated Mobile Pages) websites. This is supported only for the SCORE integration type."]
    pub fn allow_amp_traffic(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_amp_traffic", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_domains` after provisioning.\nDomains or subdomains of websites allowed to use the key. All subdomains of an allowed domain are automatically allowed. A valid domain requires a host and must not include any path, port, query or fragment. Examples: 'example.com' or 'subdomain.example.com'"]
    pub fn allowed_domains(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_domains", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `challenge_security_preference` after provisioning.\nSettings for the frequency and difficulty at which this key triggers captcha challenges. This should only be specified for IntegrationTypes CHECKBOX and INVISIBLE. Possible values: CHALLENGE_SECURITY_PREFERENCE_UNSPECIFIED, USABILITY, BALANCE, SECURITY"]
    pub fn challenge_security_preference(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.challenge_security_preference", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `integration_type` after provisioning.\nRequired. Describes how this key is integrated with the website. Possible values: SCORE, CHECKBOX, INVISIBLE, POLICY_BASED_CHALLENGE"]
    pub fn integration_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.integration_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `challenge_settings` after provisioning.\n"]
    pub fn challenge_settings(
        &self,
    ) -> ListRef<RecaptchaEnterpriseKeyWebSettingsElChallengeSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.challenge_settings", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct RecaptchaEnterpriseKeyDynamic {
    android_settings: Option<DynamicBlock<RecaptchaEnterpriseKeyAndroidSettingsEl>>,
    ios_settings: Option<DynamicBlock<RecaptchaEnterpriseKeyIosSettingsEl>>,
    testing_options: Option<DynamicBlock<RecaptchaEnterpriseKeyTestingOptionsEl>>,
    waf_settings: Option<DynamicBlock<RecaptchaEnterpriseKeyWafSettingsEl>>,
    web_settings: Option<DynamicBlock<RecaptchaEnterpriseKeyWebSettingsEl>>,
}
