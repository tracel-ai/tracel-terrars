use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AccessContextManagerAccessLevelData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    parent: PrimField<String>,
    title: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    basic: Option<Vec<AccessContextManagerAccessLevelBasicEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom: Option<Vec<AccessContextManagerAccessLevelCustomEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AccessContextManagerAccessLevelTimeoutsEl>,
    dynamic: AccessContextManagerAccessLevelDynamic,
}
struct AccessContextManagerAccessLevel_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AccessContextManagerAccessLevelData>,
}
#[derive(Clone)]
pub struct AccessContextManagerAccessLevel(Rc<AccessContextManagerAccessLevel_>);
impl AccessContextManagerAccessLevel {
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
    #[doc = "Set the field `description`.\nDescription of the AccessLevel and its use. Does not affect behavior."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerAccessLevelBasicEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().basic = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.basic = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `custom`.\n"]
    pub fn set_custom(
        self,
        v: impl Into<BlockAssignable<AccessContextManagerAccessLevelCustomEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AccessContextManagerAccessLevelTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the AccessLevel and its use. Does not affect behavior."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name for the Access Level. The short_name component must begin\nwith a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this AccessLevel lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> ListRef<AccessContextManagerAccessLevelBasicElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(&self) -> ListRef<AccessContextManagerAccessLevelCustomElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAccessLevelTimeoutsElRef {
        AccessContextManagerAccessLevelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AccessContextManagerAccessLevel {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AccessContextManagerAccessLevel {}
impl ToListMappable for AccessContextManagerAccessLevel {
    type O = ListRef<AccessContextManagerAccessLevelRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AccessContextManagerAccessLevel_ {
    fn extract_resource_type(&self) -> String {
        "google_access_context_manager_access_level".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAccessContextManagerAccessLevel {
    pub tf_id: String,
    #[doc = "Resource name for the Access Level. The short_name component must begin\nwith a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub name: PrimField<String>,
    #[doc = "The AccessPolicy this AccessLevel lives in.\nFormat: accessPolicies/{policy_id}"]
    pub parent: PrimField<String>,
    #[doc = "Human readable title. Must be unique within the Policy."]
    pub title: PrimField<String>,
}
impl BuildAccessContextManagerAccessLevel {
    pub fn build(self, stack: &mut Stack) -> AccessContextManagerAccessLevel {
        let out = AccessContextManagerAccessLevel(Rc::new(AccessContextManagerAccessLevel_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AccessContextManagerAccessLevelData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                parent: self.parent,
                title: self.title,
                basic: core::default::Default::default(),
                custom: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AccessContextManagerAccessLevelRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AccessContextManagerAccessLevelRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `deletion_policy` after provisioning.\nWhether Terraform will be prevented from destroying the instance. Defaults to \"DELETE\".\nWhen a 'terraform destroy' or 'terraform apply' would delete the instance,\nthe command will fail if this field is set to \"PREVENT\" in Terraform state.\nWhen set to \"ABANDON\", the command will remove the resource from Terraform\nmanagement without updating or deleting the resource in the API.\nWhen set to \"DELETE\", deleting the resource is allowed.\n"]
    pub fn deletion_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.deletion_policy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the AccessLevel and its use. Does not affect behavior."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nResource name for the Access Level. The short_name component must begin\nwith a letter and only include alphanumeric and '_'.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `parent` after provisioning.\nThe AccessPolicy this AccessLevel lives in.\nFormat: accessPolicies/{policy_id}"]
    pub fn parent(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.parent", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nHuman readable title. Must be unique within the Policy."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.title", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> ListRef<AccessContextManagerAccessLevelBasicElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.basic", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `custom` after provisioning.\n"]
    pub fn custom(&self) -> ListRef<AccessContextManagerAccessLevelCustomElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.custom", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AccessContextManagerAccessLevelTimeoutsElRef {
        AccessContextManagerAccessLevelTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_version: Option<PrimField<String>>,
    os_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_verified_chrome_os: Option<PrimField<bool>>,
}
impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    #[doc = "Set the field `minimum_version`.\nThe minimum allowed OS version. If not set, any version\nof this OS satisfies the constraint.\nFormat: \"major.minor.patch\" such as \"10.5.301\", \"9.2.1\"."]
    pub fn set_minimum_version(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.minimum_version = Some(v.into());
        self
    }
    #[doc = "Set the field `require_verified_chrome_os`.\nIf you specify DESKTOP_CHROME_OS for osType, you can optionally include requireVerifiedChromeOs to require Chrome Verified Access."]
    pub fn set_require_verified_chrome_os(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_verified_chrome_os = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl
{
    type O = BlockAssignable<
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    #[doc = "The operating system type of the device. Possible values: [\"OS_UNSPECIFIED\", \"DESKTOP_MAC\", \"DESKTOP_WINDOWS\", \"DESKTOP_LINUX\", \"DESKTOP_CHROME_OS\", \"ANDROID\", \"IOS\"]"]
    pub os_type: PrimField<String>,
}
impl BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
    pub fn build(
        self,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl {
            minimum_version: core::default::Default::default(),
            os_type: self.os_type,
            require_verified_chrome_os: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `minimum_version` after provisioning.\nThe minimum allowed OS version. If not set, any version\nof this OS satisfies the constraint.\nFormat: \"major.minor.patch\" such as \"10.5.301\", \"9.2.1\"."]
    pub fn minimum_version(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.minimum_version", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `os_type` after provisioning.\nThe operating system type of the device. Possible values: [\"OS_UNSPECIFIED\", \"DESKTOP_MAC\", \"DESKTOP_WINDOWS\", \"DESKTOP_LINUX\", \"DESKTOP_CHROME_OS\", \"ANDROID\", \"IOS\"]"]
    pub fn os_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.os_type", self.base))
    }
    #[doc = "Get a reference to the value of field `require_verified_chrome_os` after provisioning.\nIf you specify DESKTOP_CHROME_OS for osType, you can optionally include requireVerifiedChromeOs to require Chrome Verified Access."]
    pub fn require_verified_chrome_os(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_verified_chrome_os", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElDynamic {
    os_constraints: Option<
        DynamicBlock<
            AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_device_management_levels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_encryption_statuses: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_admin_approval: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_corp_owned: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    require_screen_lock: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    os_constraints: Option<
        Vec<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl>,
    >,
    dynamic: AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElDynamic,
}
impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    #[doc = "Set the field `allowed_device_management_levels`.\nA list of allowed device management levels.\nAn empty list allows all management levels. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"NONE\", \"BASIC\", \"COMPLETE\"]"]
    pub fn set_allowed_device_management_levels(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_device_management_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `allowed_encryption_statuses`.\nA list of allowed encryptions statuses.\nAn empty list allows all statuses. Possible values: [\"ENCRYPTION_UNSPECIFIED\", \"ENCRYPTION_UNSUPPORTED\", \"UNENCRYPTED\", \"ENCRYPTED\"]"]
    pub fn set_allowed_encryption_statuses(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.allowed_encryption_statuses = Some(v.into());
        self
    }
    #[doc = "Set the field `require_admin_approval`.\nWhether the device needs to be approved by the customer admin."]
    pub fn set_require_admin_approval(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_admin_approval = Some(v.into());
        self
    }
    #[doc = "Set the field `require_corp_owned`.\nWhether the device needs to be corp owned."]
    pub fn set_require_corp_owned(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_corp_owned = Some(v.into());
        self
    }
    #[doc = "Set the field `require_screen_lock`.\nWhether or not screenlock is required for the DevicePolicy\nto be true. Defaults to false."]
    pub fn set_require_screen_lock(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.require_screen_lock = Some(v.into());
        self
    }
    #[doc = "Set the field `os_constraints`.\n"]
    pub fn set_os_constraints(
        mut self,
        v: impl Into<
            BlockAssignable<
                AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.os_constraints = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.os_constraints = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {}
impl BuildAccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl {
            allowed_device_management_levels: core::default::Default::default(),
            allowed_encryption_statuses: core::default::Default::default(),
            require_admin_approval: core::default::Default::default(),
            require_corp_owned: core::default::Default::default(),
            require_screen_lock: core::default::Default::default(),
            os_constraints: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
        AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `allowed_device_management_levels` after provisioning.\nA list of allowed device management levels.\nAn empty list allows all management levels. Possible values: [\"MANAGEMENT_UNSPECIFIED\", \"NONE\", \"BASIC\", \"COMPLETE\"]"]
    pub fn allowed_device_management_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_device_management_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `allowed_encryption_statuses` after provisioning.\nA list of allowed encryptions statuses.\nAn empty list allows all statuses. Possible values: [\"ENCRYPTION_UNSPECIFIED\", \"ENCRYPTION_UNSUPPORTED\", \"UNENCRYPTED\", \"ENCRYPTED\"]"]
    pub fn allowed_encryption_statuses(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.allowed_encryption_statuses", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_admin_approval` after provisioning.\nWhether the device needs to be approved by the customer admin."]
    pub fn require_admin_approval(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_admin_approval", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_corp_owned` after provisioning.\nWhether the device needs to be corp owned."]
    pub fn require_corp_owned(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_corp_owned", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `require_screen_lock` after provisioning.\nWhether or not screenlock is required for the DevicePolicy\nto be true. Defaults to false."]
    pub fn require_screen_lock(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.require_screen_lock", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `os_constraints` after provisioning.\n"]
    pub fn os_constraints(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElOsConstraintsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.os_constraints", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    network: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_ip_subnetworks: Option<ListField<PrimField<String>>>,
}
impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    #[doc = "Set the field `vpc_ip_subnetworks`.\nA list of CIDR block IP subnetwork specification. Must be IPv4."]
    pub fn set_vpc_ip_subnetworks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.vpc_ip_subnetworks = Some(v.into());
        self
    }
}
impl ToListMappable
    for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl
{
    type O = BlockAssignable<
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl
{
    #[doc = "Required. Network name to be allowed by this Access Level. Networks of foreign organizations requires 'compute.network.get' permission to be granted to caller."]
    pub network: PrimField<String>,
}
impl BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
    pub fn build(
        self,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl {
            network: self.network,
            vpc_ip_subnetworks: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
    shared: StackShared,
    base: String,
}
impl Ref
    for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef
{
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef
    {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `network` after provisioning.\nRequired. Network name to be allowed by this Access Level. Networks of foreign organizations requires 'compute.network.get' permission to be granted to caller."]
    pub fn network(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network", self.base))
    }
    #[doc = "Get a reference to the value of field `vpc_ip_subnetworks` after provisioning.\nA list of CIDR block IP subnetwork specification. Must be IPv4."]
    pub fn vpc_ip_subnetworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_ip_subnetworks", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElDynamic {
    vpc_subnetwork: Option<
        DynamicBlock<
            AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_subnetwork: Option<
        Vec<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl>,
    >,
    dynamic: AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElDynamic,
}
impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    #[doc = "Set the field `vpc_subnetwork`.\n"]
    pub fn set_vpc_subnetwork(
        mut self,
        v : impl Into < BlockAssignable < AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_subnetwork = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_subnetwork = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {}
impl BuildAccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl {
            vpc_subnetwork: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `vpc_subnetwork` after provisioning.\n"]
    pub fn vpc_subnetwork(
        &self,
    ) -> ListRef<
        AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElVpcSubnetworkElRef,
    > {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_subnetwork", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElConditionsElDynamic {
    device_policy:
        Option<DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>>,
    vpc_network_sources:
        Option<DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicElConditionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_subnetworks: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    members: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    negate: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regions: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    required_access_levels: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_policy: Option<Vec<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vpc_network_sources:
        Option<Vec<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>>,
    dynamic: AccessContextManagerAccessLevelBasicElConditionsElDynamic,
}
impl AccessContextManagerAccessLevelBasicElConditionsEl {
    #[doc = "Set the field `ip_subnetworks`.\nA list of CIDR block IP subnetwork specification. May be IPv4\nor IPv6.\nNote that for a CIDR IP address block, the specified IP address\nportion must be properly truncated (i.e. all the host bits must\nbe zero) or the input is considered malformed. For example,\n\"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly,\nfor IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\"\nis not. The originating IP of a request must be in one of the\nlisted subnets in order for this Condition to be true.\nIf empty, all IP addresses are allowed."]
    pub fn set_ip_subnetworks(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.ip_subnetworks = Some(v.into());
        self
    }
    #[doc = "Set the field `members`.\nAn allowed list of members (users, service accounts).\nUsing groups is not supported yet.\n\nThe signed-in user originating the request must be a part of one\nof the provided members. If not specified, a request may come\nfrom any user (logged in/not logged in, not present in any\ngroups, etc.).\nFormats: 'user:{emailid}', 'serviceAccount:{emailid}'"]
    pub fn set_members(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.members = Some(v.into());
        self
    }
    #[doc = "Set the field `negate`.\nWhether to negate the Condition. If true, the Condition becomes\na NAND over its non-empty fields, each field must be false for\nthe Condition overall to be satisfied. Defaults to false."]
    pub fn set_negate(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.negate = Some(v.into());
        self
    }
    #[doc = "Set the field `regions`.\nThe request must originate from one of the provided\ncountries/regions.\nFormat: A valid ISO 3166-1 alpha-2 code."]
    pub fn set_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.regions = Some(v.into());
        self
    }
    #[doc = "Set the field `required_access_levels`.\nA list of other access levels defined in the same Policy,\nreferenced by resource name. Referencing an AccessLevel which\ndoes not exist is an error. All access levels listed must be\ngranted for the Condition to be true.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn set_required_access_levels(
        mut self,
        v: impl Into<ListField<PrimField<String>>>,
    ) -> Self {
        self.required_access_levels = Some(v.into());
        self
    }
    #[doc = "Set the field `device_policy`.\n"]
    pub fn set_device_policy(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.device_policy = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.device_policy = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vpc_network_sources`.\n"]
    pub fn set_vpc_network_sources(
        mut self,
        v: impl Into<
            BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vpc_network_sources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_network_sources = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelBasicElConditionsEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelBasicElConditionsEl {}
impl BuildAccessContextManagerAccessLevelBasicElConditionsEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicElConditionsEl {
        AccessContextManagerAccessLevelBasicElConditionsEl {
            ip_subnetworks: core::default::Default::default(),
            members: core::default::Default::default(),
            negate: core::default::Default::default(),
            regions: core::default::Default::default(),
            required_access_levels: core::default::Default::default(),
            device_policy: core::default::Default::default(),
            vpc_network_sources: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelBasicElConditionsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelBasicElConditionsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AccessContextManagerAccessLevelBasicElConditionsElRef {
        AccessContextManagerAccessLevelBasicElConditionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelBasicElConditionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ip_subnetworks` after provisioning.\nA list of CIDR block IP subnetwork specification. May be IPv4\nor IPv6.\nNote that for a CIDR IP address block, the specified IP address\nportion must be properly truncated (i.e. all the host bits must\nbe zero) or the input is considered malformed. For example,\n\"192.0.2.0/24\" is accepted but \"192.0.2.1/24\" is not. Similarly,\nfor IPv6, \"2001:db8::/32\" is accepted whereas \"2001:db8::1/32\"\nis not. The originating IP of a request must be in one of the\nlisted subnets in order for this Condition to be true.\nIf empty, all IP addresses are allowed."]
    pub fn ip_subnetworks(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ip_subnetworks", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `members` after provisioning.\nAn allowed list of members (users, service accounts).\nUsing groups is not supported yet.\n\nThe signed-in user originating the request must be a part of one\nof the provided members. If not specified, a request may come\nfrom any user (logged in/not logged in, not present in any\ngroups, etc.).\nFormats: 'user:{emailid}', 'serviceAccount:{emailid}'"]
    pub fn members(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.members", self.base))
    }
    #[doc = "Get a reference to the value of field `negate` after provisioning.\nWhether to negate the Condition. If true, the Condition becomes\na NAND over its non-empty fields, each field must be false for\nthe Condition overall to be satisfied. Defaults to false."]
    pub fn negate(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.negate", self.base))
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\nThe request must originate from one of the provided\ncountries/regions.\nFormat: A valid ISO 3166-1 alpha-2 code."]
    pub fn regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
    #[doc = "Get a reference to the value of field `required_access_levels` after provisioning.\nA list of other access levels defined in the same Policy,\nreferenced by resource name. Referencing an AccessLevel which\ndoes not exist is an error. All access levels listed must be\ngranted for the Condition to be true.\nFormat: accessPolicies/{policy_id}/accessLevels/{short_name}"]
    pub fn required_access_levels(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.required_access_levels", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `device_policy` after provisioning.\n"]
    pub fn device_policy(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelBasicElConditionsElDevicePolicyElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.device_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vpc_network_sources` after provisioning.\n"]
    pub fn vpc_network_sources(
        &self,
    ) -> ListRef<AccessContextManagerAccessLevelBasicElConditionsElVpcNetworkSourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vpc_network_sources", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelBasicElDynamic {
    conditions: Option<DynamicBlock<AccessContextManagerAccessLevelBasicElConditionsEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelBasicEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    combining_function: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditions: Option<Vec<AccessContextManagerAccessLevelBasicElConditionsEl>>,
    dynamic: AccessContextManagerAccessLevelBasicElDynamic,
}
impl AccessContextManagerAccessLevelBasicEl {
    #[doc = "Set the field `combining_function`.\nHow the conditions list should be combined to determine if a request\nis granted this AccessLevel. If AND is used, each Condition in\nconditions must be satisfied for the AccessLevel to be applied. If\nOR is used, at least one Condition in conditions must be satisfied\nfor the AccessLevel to be applied. Default value: \"AND\" Possible values: [\"AND\", \"OR\"]"]
    pub fn set_combining_function(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.combining_function = Some(v.into());
        self
    }
    #[doc = "Set the field `conditions`.\n"]
    pub fn set_conditions(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerAccessLevelBasicElConditionsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.conditions = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.conditions = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelBasicEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelBasicEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelBasicEl {}
impl BuildAccessContextManagerAccessLevelBasicEl {
    pub fn build(self) -> AccessContextManagerAccessLevelBasicEl {
        AccessContextManagerAccessLevelBasicEl {
            combining_function: core::default::Default::default(),
            conditions: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelBasicElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelBasicElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerAccessLevelBasicElRef {
        AccessContextManagerAccessLevelBasicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelBasicElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `combining_function` after provisioning.\nHow the conditions list should be combined to determine if a request\nis granted this AccessLevel. If AND is used, each Condition in\nconditions must be satisfied for the AccessLevel to be applied. If\nOR is used, at least one Condition in conditions must be satisfied\nfor the AccessLevel to be applied. Default value: \"AND\" Possible values: [\"AND\", \"OR\"]"]
    pub fn combining_function(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.combining_function", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `conditions` after provisioning.\n"]
    pub fn conditions(&self) -> ListRef<AccessContextManagerAccessLevelBasicElConditionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.conditions", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelCustomElExprEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    expression: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<PrimField<String>>,
}
impl AccessContextManagerAccessLevelCustomElExprEl {
    #[doc = "Set the field `description`.\nDescription of the expression"]
    pub fn set_description(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.description = Some(v.into());
        self
    }
    #[doc = "Set the field `location`.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn set_location(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.location = Some(v.into());
        self
    }
    #[doc = "Set the field `title`.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn set_title(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.title = Some(v.into());
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelCustomElExprEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelCustomElExprEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelCustomElExprEl {
    #[doc = "Textual representation of an expression in Common Expression Language syntax."]
    pub expression: PrimField<String>,
}
impl BuildAccessContextManagerAccessLevelCustomElExprEl {
    pub fn build(self) -> AccessContextManagerAccessLevelCustomElExprEl {
        AccessContextManagerAccessLevelCustomElExprEl {
            description: core::default::Default::default(),
            expression: self.expression,
            location: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelCustomElExprElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelCustomElExprElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerAccessLevelCustomElExprElRef {
        AccessContextManagerAccessLevelCustomElExprElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelCustomElExprElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `description` after provisioning.\nDescription of the expression"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }
    #[doc = "Get a reference to the value of field `expression` after provisioning.\nTextual representation of an expression in Common Expression Language syntax."]
    pub fn expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nString indicating the location of the expression for error reporting, e.g. a file name and a position in the file"]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }
    #[doc = "Get a reference to the value of field `title` after provisioning.\nTitle for the expression, i.e. a short string describing its purpose."]
    pub fn title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}
#[derive(Serialize, Default)]
struct AccessContextManagerAccessLevelCustomElDynamic {
    expr: Option<DynamicBlock<AccessContextManagerAccessLevelCustomElExprEl>>,
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelCustomEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    expr: Option<Vec<AccessContextManagerAccessLevelCustomElExprEl>>,
    dynamic: AccessContextManagerAccessLevelCustomElDynamic,
}
impl AccessContextManagerAccessLevelCustomEl {
    #[doc = "Set the field `expr`.\n"]
    pub fn set_expr(
        mut self,
        v: impl Into<BlockAssignable<AccessContextManagerAccessLevelCustomElExprEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.expr = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.expr = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for AccessContextManagerAccessLevelCustomEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelCustomEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelCustomEl {}
impl BuildAccessContextManagerAccessLevelCustomEl {
    pub fn build(self) -> AccessContextManagerAccessLevelCustomEl {
        AccessContextManagerAccessLevelCustomEl {
            expr: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelCustomElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelCustomElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerAccessLevelCustomElRef {
        AccessContextManagerAccessLevelCustomElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelCustomElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `expr` after provisioning.\n"]
    pub fn expr(&self) -> ListRef<AccessContextManagerAccessLevelCustomElExprElRef> {
        ListRef::new(self.shared().clone(), format!("{}.expr", self.base))
    }
}
#[derive(Serialize)]
pub struct AccessContextManagerAccessLevelTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AccessContextManagerAccessLevelTimeoutsEl {
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
impl ToListMappable for AccessContextManagerAccessLevelTimeoutsEl {
    type O = BlockAssignable<AccessContextManagerAccessLevelTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAccessContextManagerAccessLevelTimeoutsEl {}
impl BuildAccessContextManagerAccessLevelTimeoutsEl {
    pub fn build(self) -> AccessContextManagerAccessLevelTimeoutsEl {
        AccessContextManagerAccessLevelTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AccessContextManagerAccessLevelTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AccessContextManagerAccessLevelTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AccessContextManagerAccessLevelTimeoutsElRef {
        AccessContextManagerAccessLevelTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AccessContextManagerAccessLevelTimeoutsElRef {
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
struct AccessContextManagerAccessLevelDynamic {
    basic: Option<DynamicBlock<AccessContextManagerAccessLevelBasicEl>>,
    custom: Option<DynamicBlock<AccessContextManagerAccessLevelCustomEl>>,
}
