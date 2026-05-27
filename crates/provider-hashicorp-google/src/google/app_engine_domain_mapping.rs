use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct AppEngineDomainMappingData {
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
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssl_settings: Option<Vec<AppEngineDomainMappingSslSettingsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<AppEngineDomainMappingTimeoutsEl>,
    dynamic: AppEngineDomainMappingDynamic,
}
struct AppEngineDomainMapping_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppEngineDomainMappingData>,
}
#[derive(Clone)]
pub struct AppEngineDomainMapping(Rc<AppEngineDomainMapping_>);
impl AppEngineDomainMapping {
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
    #[doc = "Set the field `override_strategy`.\nWhether the domain creation should override any existing mappings for this domain.\nBy default, overrides are rejected. Default value: \"STRICT\" Possible values: [\"STRICT\", \"OVERRIDE\"]"]
    pub fn set_override_strategy(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().override_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `project`.\n"]
    pub fn set_project(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().project = Some(v.into());
        self
    }
    #[doc = "Set the field `ssl_settings`.\n"]
    pub fn set_ssl_settings(
        self,
        v: impl Into<BlockAssignable<AppEngineDomainMappingSslSettingsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_settings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_settings = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<AppEngineDomainMappingTimeoutsEl>) -> Self {
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
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nRelative name of the domain serving the application. Example: example.com."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_strategy` after provisioning.\nWhether the domain creation should override any existing mappings for this domain.\nBy default, overrides are rejected. Default value: \"STRICT\" Possible values: [\"STRICT\", \"OVERRIDE\"]"]
    pub fn override_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_records` after provisioning.\nThe resource records required to configure this domain mapping. These records must be added to the domain's DNS\nconfiguration in order to serve the application via this domain mapping."]
    pub fn resource_records(&self) -> ListRef<AppEngineDomainMappingResourceRecordsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_records", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_settings` after provisioning.\n"]
    pub fn ssl_settings(&self) -> ListRef<AppEngineDomainMappingSslSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineDomainMappingTimeoutsElRef {
        AppEngineDomainMappingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for AppEngineDomainMapping {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for AppEngineDomainMapping {}
impl ToListMappable for AppEngineDomainMapping {
    type O = ListRef<AppEngineDomainMappingRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for AppEngineDomainMapping_ {
    fn extract_resource_type(&self) -> String {
        "google_app_engine_domain_mapping".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildAppEngineDomainMapping {
    pub tf_id: String,
    #[doc = "Relative name of the domain serving the application. Example: example.com."]
    pub domain_name: PrimField<String>,
}
impl BuildAppEngineDomainMapping {
    pub fn build(self, stack: &mut Stack) -> AppEngineDomainMapping {
        let out = AppEngineDomainMapping(Rc::new(AppEngineDomainMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppEngineDomainMappingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                override_strategy: core::default::Default::default(),
                project: core::default::Default::default(),
                ssl_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct AppEngineDomainMappingRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineDomainMappingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl AppEngineDomainMappingRef {
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
    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\nRelative name of the domain serving the application. Example: example.com."]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.domain_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nFull path to the DomainMapping resource in the API. Example: apps/myapp/domainMapping/example.com."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `override_strategy` after provisioning.\nWhether the domain creation should override any existing mappings for this domain.\nBy default, overrides are rejected. Default value: \"STRICT\" Possible values: [\"STRICT\", \"OVERRIDE\"]"]
    pub fn override_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.override_strategy", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `resource_records` after provisioning.\nThe resource records required to configure this domain mapping. These records must be added to the domain's DNS\nconfiguration in order to serve the application via this domain mapping."]
    pub fn resource_records(&self) -> ListRef<AppEngineDomainMappingResourceRecordsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.resource_records", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_settings` after provisioning.\n"]
    pub fn ssl_settings(&self) -> ListRef<AppEngineDomainMappingSslSettingsElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ssl_settings", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AppEngineDomainMappingTimeoutsElRef {
        AppEngineDomainMappingTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineDomainMappingResourceRecordsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rrdata: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}
impl AppEngineDomainMappingResourceRecordsEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
    #[doc = "Set the field `rrdata`.\n"]
    pub fn set_rrdata(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.rrdata = Some(v.into());
        self
    }
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineDomainMappingResourceRecordsEl {
    type O = BlockAssignable<AppEngineDomainMappingResourceRecordsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineDomainMappingResourceRecordsEl {}
impl BuildAppEngineDomainMappingResourceRecordsEl {
    pub fn build(self) -> AppEngineDomainMappingResourceRecordsEl {
        AppEngineDomainMappingResourceRecordsEl {
            name: core::default::Default::default(),
            rrdata: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}
pub struct AppEngineDomainMappingResourceRecordsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineDomainMappingResourceRecordsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineDomainMappingResourceRecordsElRef {
        AppEngineDomainMappingResourceRecordsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineDomainMappingResourceRecordsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `rrdata` after provisioning.\n"]
    pub fn rrdata(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rrdata", self.base))
    }
    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}
#[derive(Serialize)]
pub struct AppEngineDomainMappingSslSettingsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    certificate_id: Option<PrimField<String>>,
    ssl_management_type: PrimField<String>,
}
impl AppEngineDomainMappingSslSettingsEl {
    #[doc = "Set the field `certificate_id`.\nID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will\nremove SSL support.\nBy default, a managed certificate is automatically created for every domain mapping. To omit SSL support\nor to configure SSL manually, specify 'SslManagementType.MANUAL' on a 'CREATE' or 'UPDATE' request. You must be\nauthorized to administer the 'AuthorizedCertificate' resource to manually map it to a DomainMapping resource.\nExample: 12345."]
    pub fn set_certificate_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.certificate_id = Some(v.into());
        self
    }
}
impl ToListMappable for AppEngineDomainMappingSslSettingsEl {
    type O = BlockAssignable<AppEngineDomainMappingSslSettingsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineDomainMappingSslSettingsEl {
    #[doc = "SSL management type for this domain. If 'AUTOMATIC', a managed certificate is automatically provisioned.\nIf 'MANUAL', 'certificateId' must be manually specified in order to configure SSL for this domain. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub ssl_management_type: PrimField<String>,
}
impl BuildAppEngineDomainMappingSslSettingsEl {
    pub fn build(self) -> AppEngineDomainMappingSslSettingsEl {
        AppEngineDomainMappingSslSettingsEl {
            certificate_id: core::default::Default::default(),
            ssl_management_type: self.ssl_management_type,
        }
    }
}
pub struct AppEngineDomainMappingSslSettingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineDomainMappingSslSettingsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineDomainMappingSslSettingsElRef {
        AppEngineDomainMappingSslSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineDomainMappingSslSettingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `certificate_id` after provisioning.\nID of the AuthorizedCertificate resource configuring SSL for the application. Clearing this field will\nremove SSL support.\nBy default, a managed certificate is automatically created for every domain mapping. To omit SSL support\nor to configure SSL manually, specify 'SslManagementType.MANUAL' on a 'CREATE' or 'UPDATE' request. You must be\nauthorized to administer the 'AuthorizedCertificate' resource to manually map it to a DomainMapping resource.\nExample: 12345."]
    pub fn certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `pending_managed_certificate_id` after provisioning.\nID of the managed 'AuthorizedCertificate' resource currently being provisioned, if applicable. Until the new\nmanaged certificate has been successfully provisioned, the previous SSL state will be preserved. Once the\nprovisioning process completes, the 'certificateId' field will reflect the new managed certificate and this\nfield will be left empty. To remove SSL support while there is still a pending managed certificate, clear the\n'certificateId' field with an update request."]
    pub fn pending_managed_certificate_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.pending_managed_certificate_id", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `ssl_management_type` after provisioning.\nSSL management type for this domain. If 'AUTOMATIC', a managed certificate is automatically provisioned.\nIf 'MANUAL', 'certificateId' must be manually specified in order to configure SSL for this domain. Possible values: [\"AUTOMATIC\", \"MANUAL\"]"]
    pub fn ssl_management_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.ssl_management_type", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct AppEngineDomainMappingTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl AppEngineDomainMappingTimeoutsEl {
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
impl ToListMappable for AppEngineDomainMappingTimeoutsEl {
    type O = BlockAssignable<AppEngineDomainMappingTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildAppEngineDomainMappingTimeoutsEl {}
impl BuildAppEngineDomainMappingTimeoutsEl {
    pub fn build(self) -> AppEngineDomainMappingTimeoutsEl {
        AppEngineDomainMappingTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct AppEngineDomainMappingTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for AppEngineDomainMappingTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> AppEngineDomainMappingTimeoutsElRef {
        AppEngineDomainMappingTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl AppEngineDomainMappingTimeoutsElRef {
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
struct AppEngineDomainMappingDynamic {
    ssl_settings: Option<DynamicBlock<AppEngineDomainMappingSslSettingsEl>>,
}
