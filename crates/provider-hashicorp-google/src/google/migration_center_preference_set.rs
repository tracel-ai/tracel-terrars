use super::provider::ProviderGoogle;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct MigrationCenterPreferenceSetData {
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
    display_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    location: PrimField<String>,
    preference_set_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<MigrationCenterPreferenceSetTimeoutsEl>,
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_machine_preferences:
        Option<Vec<MigrationCenterPreferenceSetVirtualMachinePreferencesEl>>,
    dynamic: MigrationCenterPreferenceSetDynamic,
}
struct MigrationCenterPreferenceSet_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<MigrationCenterPreferenceSetData>,
}
#[derive(Clone)]
pub struct MigrationCenterPreferenceSet(Rc<MigrationCenterPreferenceSet_>);
impl MigrationCenterPreferenceSet {
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
    #[doc = "Set the field `description`.\nA description of the preference set."]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }
    #[doc = "Set the field `display_name`.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn set_display_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().display_name = Some(v.into());
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
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<MigrationCenterPreferenceSetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Set the field `virtual_machine_preferences`.\n"]
    pub fn set_virtual_machine_preferences(
        self,
        v: impl Into<BlockAssignable<MigrationCenterPreferenceSetVirtualMachinePreferencesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().virtual_machine_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.virtual_machine_preferences = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the preference set was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the preference set."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the preference set."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preference_set_id` after provisioning.\nRequired. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression '[a-z]([a-z0-9-]{0,61}[a-z0-9])?'."]
    pub fn preference_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preference_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp when the preference set was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterPreferenceSetTimeoutsElRef {
        MigrationCenterPreferenceSetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_machine_preferences` after provisioning.\n"]
    pub fn virtual_machine_preferences(
        &self,
    ) -> ListRef<MigrationCenterPreferenceSetVirtualMachinePreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_machine_preferences", self.extract_ref()),
        )
    }
}
impl Referable for MigrationCenterPreferenceSet {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for MigrationCenterPreferenceSet {}
impl ToListMappable for MigrationCenterPreferenceSet {
    type O = ListRef<MigrationCenterPreferenceSetRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for MigrationCenterPreferenceSet_ {
    fn extract_resource_type(&self) -> String {
        "google_migration_center_preference_set".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildMigrationCenterPreferenceSet {
    pub tf_id: String,
    #[doc = "Part of 'parent'. See documentation of 'projectsId'."]
    pub location: PrimField<String>,
    #[doc = "Required. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression '[a-z]([a-z0-9-]{0,61}[a-z0-9])?'."]
    pub preference_set_id: PrimField<String>,
}
impl BuildMigrationCenterPreferenceSet {
    pub fn build(self, stack: &mut Stack) -> MigrationCenterPreferenceSet {
        let out = MigrationCenterPreferenceSet(Rc::new(MigrationCenterPreferenceSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(MigrationCenterPreferenceSetData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                deletion_policy: core::default::Default::default(),
                description: core::default::Default::default(),
                display_name: core::default::Default::default(),
                id: core::default::Default::default(),
                location: self.location,
                preference_set_id: self.preference_set_id,
                project: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                virtual_machine_preferences: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct MigrationCenterPreferenceSetRef {
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl MigrationCenterPreferenceSetRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create_time` after provisioning.\nOutput only. The timestamp when the preference set was created."]
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
    #[doc = "Get a reference to the value of field `description` after provisioning.\nA description of the preference set."]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.description", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `display_name` after provisioning.\nUser-friendly display name. Maximum length is 63 characters."]
    pub fn display_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.display_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `location` after provisioning.\nPart of 'parent'. See documentation of 'projectsId'."]
    pub fn location(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.location", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\nOutput only. Name of the preference set."]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `preference_set_id` after provisioning.\nRequired. User specified ID for the preference set. It will become the last component of the preference set name. The ID must be unique within the project, must conform with RFC-1034, is restricted to lower-cased letters, and has a maximum length of 63 characters. The ID must match the regular expression '[a-z]([a-z0-9-]{0,61}[a-z0-9])?'."]
    pub fn preference_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.preference_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.project", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `update_time` after provisioning.\nOutput only. The timestamp when the preference set was last updated."]
    pub fn update_time(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.update_time", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MigrationCenterPreferenceSetTimeoutsElRef {
        MigrationCenterPreferenceSetTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `virtual_machine_preferences` after provisioning.\n"]
    pub fn virtual_machine_preferences(
        &self,
    ) -> ListRef<MigrationCenterPreferenceSetVirtualMachinePreferencesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.virtual_machine_preferences", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl MigrationCenterPreferenceSetTimeoutsEl {
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
impl ToListMappable for MigrationCenterPreferenceSetTimeoutsEl {
    type O = BlockAssignable<MigrationCenterPreferenceSetTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMigrationCenterPreferenceSetTimeoutsEl {}
impl BuildMigrationCenterPreferenceSetTimeoutsEl {
    pub fn build(self) -> MigrationCenterPreferenceSetTimeoutsEl {
        MigrationCenterPreferenceSetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct MigrationCenterPreferenceSetTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> MigrationCenterPreferenceSetTimeoutsElRef {
        MigrationCenterPreferenceSetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MigrationCenterPreferenceSetTimeoutsElRef {
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
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<PrimField<String>>,
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl { # [doc = "Set the field `code`.\nCode to identify a Compute Engine machine series. Consult https://cloud.google.com/compute/docs/machine-resource#machine_type_comparison for more details on the available series."] pub fn set_code (mut self , v : impl Into < PrimField < String > >) -> Self { self . code = Some (v . into ()) ; self } }
impl ToListMappable for MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl { type O = BlockAssignable < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl
{}
impl BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl { pub fn build (self) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl { MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl { code : core :: default :: Default :: default () , } } }
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesElRef { fn new (shared : StackShared , base : String) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesElRef { MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesElRef { shared : shared , base : base . to_string () , } } }
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `code` after provisioning.\nCode to identify a Compute Engine machine series. Consult https://cloud.google.com/compute/docs/machine-resource#machine_type_comparison for more details on the available series."] pub fn code (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.code" , self . base)) } }
#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElDynamic { allowed_machine_series : Option < DynamicBlock < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl >> , }
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl { # [serde (skip_serializing_if = "Option::is_none")] allowed_machine_series : Option < Vec < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl > > , dynamic : MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElDynamic , }
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl { # [doc = "Set the field `allowed_machine_series`.\n"] pub fn set_allowed_machine_series (mut self , v : impl Into < BlockAssignable < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . allowed_machine_series = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . allowed_machine_series = Some (d) ; } } self } }
impl ToListMappable for MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl { type O = BlockAssignable < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl
{}
impl BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl { pub fn build (self) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl { MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl { allowed_machine_series : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElRef { fn new (shared : StackShared , base : String) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElRef { MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElRef { shared : shared , base : base . to_string () , } } }
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `allowed_machine_series` after provisioning.\n"] pub fn allowed_machine_series (& self) -> ListRef < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElAllowedMachineSeriesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.allowed_machine_series" , self . base)) } }
#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElDynamic { machine_preferences : Option < DynamicBlock < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl >> , }
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl { # [serde (skip_serializing_if = "Option::is_none")] license_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] persistent_disk_type : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] machine_preferences : Option < Vec < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl > > , dynamic : MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElDynamic , }
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl {
    #[doc = "Set the field `license_type`.\nLicense type to consider when calculating costs for virtual machine insights and recommendations. If unspecified, costs are calculated based on the default licensing plan. Possible values: 'LICENSE_TYPE_UNSPECIFIED', 'LICENSE_TYPE_DEFAULT', 'LICENSE_TYPE_BRING_YOUR_OWN_LICENSE'"]
    pub fn set_license_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.license_type = Some(v.into());
        self
    }
    #[doc = "Set the field `persistent_disk_type`.\nPersistent disk type to use. If unspecified (default), all types are considered, based on available usage data. Possible values: [\"PERSISTENT_DISK_TYPE_STANDARD\", \"PERSISTENT_DISK_TYPE_BALANCED\", \"PERSISTENT_DISK_TYPE_SSD\"]"]
    pub fn set_persistent_disk_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.persistent_disk_type = Some(v.into());
        self
    }
    #[doc = "Set the field `machine_preferences`.\n"]
    pub fn set_machine_preferences(
        mut self,
        v : impl Into < BlockAssignable < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.machine_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.machine_preferences = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl
{
    type O = BlockAssignable<
        MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl {
}
impl BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl {
    pub fn build(
        self,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl {
            license_type: core::default::Default::default(),
            persistent_disk_type: core::default::Default::default(),
            machine_preferences: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElRef {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `license_type` after provisioning.\nLicense type to consider when calculating costs for virtual machine insights and recommendations. If unspecified, costs are calculated based on the default licensing plan. Possible values: 'LICENSE_TYPE_UNSPECIFIED', 'LICENSE_TYPE_DEFAULT', 'LICENSE_TYPE_BRING_YOUR_OWN_LICENSE'"]
    pub fn license_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.license_type", self.base))
    }
    #[doc = "Get a reference to the value of field `persistent_disk_type` after provisioning.\nPersistent disk type to use. If unspecified (default), all types are considered, based on available usage data. Possible values: [\"PERSISTENT_DISK_TYPE_STANDARD\", \"PERSISTENT_DISK_TYPE_BALANCED\", \"PERSISTENT_DISK_TYPE_SSD\"]"]
    pub fn persistent_disk_type(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.persistent_disk_type", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `machine_preferences` after provisioning.\n"]    pub fn machine_preferences (& self) -> ListRef < MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElMachinePreferencesElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.machine_preferences", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_regions: Option<ListField<PrimField<String>>>,
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl {
    #[doc = "Set the field `preferred_regions`.\nA list of preferred regions, ordered by the most preferred region first. Set only valid Google Cloud region names. See https://cloud.google.com/compute/docs/regions-zones for available regions."]
    pub fn set_preferred_regions(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.preferred_regions = Some(v.into());
        self
    }
}
impl ToListMappable for MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl {
    type O =
        BlockAssignable<MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl {}
impl BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl {
    pub fn build(
        self,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl {
            preferred_regions: core::default::Default::default(),
        }
    }
}
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesElRef {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `preferred_regions` after provisioning.\nA list of preferred regions, ordered by the most preferred region first. Set only valid Google Cloud region names. See https://cloud.google.com/compute/docs/regions-zones for available regions."]
    pub fn preferred_regions(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.preferred_regions", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    node_name: Option<PrimField<String>>,
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl {
    #[doc = "Set the field `node_name`.\nName of the Sole Tenant node. Consult https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes"]
    pub fn set_node_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.node_name = Some(v.into());
        self
    }
}
impl ToListMappable
    for MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl
{
    type O = BlockAssignable<
        MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl
{}
impl
    BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl
{
    pub fn build(
        self,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl
    {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl {
            node_name: core::default::Default::default(),
        }
    }
}
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesElRef { fn new (shared : StackShared , base : String) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesElRef { MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesElRef { shared : shared , base : base . to_string () , } } }
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `node_name` after provisioning.\nName of the Sole Tenant node. Consult https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes"]
    pub fn node_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.node_name", self.base))
    }
}
#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElDynamic { node_types : Option < DynamicBlock < MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl >> , }
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl { # [serde (skip_serializing_if = "Option::is_none")] commitment_plan : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] cpu_overcommit_ratio : Option < PrimField < f64 > > , # [serde (skip_serializing_if = "Option::is_none")] host_maintenance_policy : Option < PrimField < String > > , # [serde (skip_serializing_if = "Option::is_none")] node_types : Option < Vec < MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl > > , dynamic : MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElDynamic , }
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl {
    #[doc = "Set the field `commitment_plan`.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR', 'COMMITMENT_3_YEAR'"]
    pub fn set_commitment_plan(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commitment_plan = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_overcommit_ratio`.\nCPU overcommit ratio. Acceptable values are between 1.0 and 2.0 inclusive."]
    pub fn set_cpu_overcommit_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_overcommit_ratio = Some(v.into());
        self
    }
    #[doc = "Set the field `host_maintenance_policy`.\nSole Tenancy nodes maintenance policy. Possible values: 'HOST_MAINTENANCE_POLICY_UNSPECIFIED', 'HOST_MAINTENANCE_POLICY_DEFAULT', 'HOST_MAINTENANCE_POLICY_RESTART_IN_PLACE', 'HOST_MAINTENANCE_POLICY_MIGRATE_WITHIN_NODE_GROUP'"]
    pub fn set_host_maintenance_policy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.host_maintenance_policy = Some(v.into());
        self
    }
    #[doc = "Set the field `node_types`.\n"]
    pub fn set_node_types(
        mut self,
        v : impl Into < BlockAssignable < MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.node_types = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.node_types = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl
{
    type O = BlockAssignable<
        MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl {}
impl BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl {
    pub fn build(
        self,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl {
            commitment_plan: core::default::Default::default(),
            cpu_overcommit_ratio: core::default::Default::default(),
            host_maintenance_policy: core::default::Default::default(),
            node_types: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElRef {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `commitment_plan` after provisioning.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR', 'COMMITMENT_3_YEAR'"]
    pub fn commitment_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_plan", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_overcommit_ratio` after provisioning.\nCPU overcommit ratio. Acceptable values are between 1.0 and 2.0 inclusive."]
    pub fn cpu_overcommit_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_overcommit_ratio", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `host_maintenance_policy` after provisioning.\nSole Tenancy nodes maintenance policy. Possible values: 'HOST_MAINTENANCE_POLICY_UNSPECIFIED', 'HOST_MAINTENANCE_POLICY_DEFAULT', 'HOST_MAINTENANCE_POLICY_RESTART_IN_PLACE', 'HOST_MAINTENANCE_POLICY_MIGRATE_WITHIN_NODE_GROUP'"]
    pub fn host_maintenance_policy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.host_maintenance_policy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `node_types` after provisioning.\n"]    pub fn node_types (& self) -> ListRef < MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElNodeTypesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.node_types", self.base))
    }
}
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment_plan: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_overcommit_ratio: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_overcommit_ratio: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_deduplication_compression_ratio: Option<PrimField<f64>>,
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl {
    #[doc = "Set the field `commitment_plan`.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_3_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_1_YEAR_UPFRONT_PAYMENT', 'COMMITMENT_3_YEAR_UPFRONT_PAYMENT',"]
    pub fn set_commitment_plan(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commitment_plan = Some(v.into());
        self
    }
    #[doc = "Set the field `cpu_overcommit_ratio`.\nCPU overcommit ratio. Acceptable values are between 1.0 and 8.0, with 0.1 increment."]
    pub fn set_cpu_overcommit_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_overcommit_ratio = Some(v.into());
        self
    }
    #[doc = "Set the field `memory_overcommit_ratio`.\nMemory overcommit ratio. Acceptable values are 1.0, 1.25, 1.5, 1.75 and 2.0."]
    pub fn set_memory_overcommit_ratio(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_overcommit_ratio = Some(v.into());
        self
    }
    #[doc = "Set the field `storage_deduplication_compression_ratio`.\nThe Deduplication and Compression ratio is based on the logical (Used Before) space required to store data before applying deduplication and compression, in relation to the physical (Used After) space required after applying deduplication and compression. Specifically, the ratio is the Used Before space divided by the Used After space. For example, if the Used Before space is 3 GB, but the physical Used After space is 1 GB, the deduplication and compression ratio is 3x. Acceptable values are between 1.0 and 4.0."]
    pub fn set_storage_deduplication_compression_ratio(
        mut self,
        v: impl Into<PrimField<f64>>,
    ) -> Self {
        self.storage_deduplication_compression_ratio = Some(v.into());
        self
    }
}
impl ToListMappable
    for MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl
{
    type O = BlockAssignable<
        MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl {
}
impl BuildMigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl {
    pub fn build(
        self,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl {
            commitment_plan: core::default::Default::default(),
            cpu_overcommit_ratio: core::default::Default::default(),
            memory_overcommit_ratio: core::default::Default::default(),
            storage_deduplication_compression_ratio: core::default::Default::default(),
        }
    }
}
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesElRef {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `commitment_plan` after provisioning.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'ON_DEMAND', 'COMMITMENT_1_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_3_YEAR_MONTHLY_PAYMENTS', 'COMMITMENT_1_YEAR_UPFRONT_PAYMENT', 'COMMITMENT_3_YEAR_UPFRONT_PAYMENT',"]
    pub fn commitment_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_plan", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `cpu_overcommit_ratio` after provisioning.\nCPU overcommit ratio. Acceptable values are between 1.0 and 8.0, with 0.1 increment."]
    pub fn cpu_overcommit_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.cpu_overcommit_ratio", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `memory_overcommit_ratio` after provisioning.\nMemory overcommit ratio. Acceptable values are 1.0, 1.25, 1.5, 1.75 and 2.0."]
    pub fn memory_overcommit_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.memory_overcommit_ratio", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `storage_deduplication_compression_ratio` after provisioning.\nThe Deduplication and Compression ratio is based on the logical (Used Before) space required to store data before applying deduplication and compression, in relation to the physical (Used After) space required after applying deduplication and compression. Specifically, the ratio is the Used Before space divided by the Used After space. For example, if the Used Before space is 3 GB, but the physical Used After space is 1 GB, the deduplication and compression ratio is 3x. Acceptable values are between 1.0 and 4.0."]
    pub fn storage_deduplication_compression_ratio(&self) -> PrimExpr<f64> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.storage_deduplication_compression_ratio", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSetVirtualMachinePreferencesElDynamic {
    compute_engine_preferences: Option<
        DynamicBlock<
            MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl,
        >,
    >,
    region_preferences: Option<
        DynamicBlock<MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl>,
    >,
    sole_tenancy_preferences: Option<
        DynamicBlock<
            MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl,
        >,
    >,
    vmware_engine_preferences: Option<
        DynamicBlock<
            MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    commitment_plan: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sizing_optimization_strategy: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_product: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_engine_preferences: Option<
        Vec<MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_preferences:
        Option<Vec<MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sole_tenancy_preferences: Option<
        Vec<MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl>,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    vmware_engine_preferences: Option<
        Vec<MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl>,
    >,
    dynamic: MigrationCenterPreferenceSetVirtualMachinePreferencesElDynamic,
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesEl {
    #[doc = "Set the field `commitment_plan`.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'COMMITMENT_PLAN_NONE', 'COMMITMENT_PLAN_ONE_YEAR', 'COMMITMENT_PLAN_THREE_YEARS'"]
    pub fn set_commitment_plan(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.commitment_plan = Some(v.into());
        self
    }
    #[doc = "Set the field `sizing_optimization_strategy`.\nSizing optimization strategy specifies the preferred strategy used when extrapolating usage data to calculate insights and recommendations for a virtual machine. If you are unsure which value to set, a moderate sizing optimization strategy is often a good value to start with. Possible values: 'SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED', 'SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE', 'SIZING_OPTIMIZATION_STRATEGY_MODERATE', 'SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE'"]
    pub fn set_sizing_optimization_strategy(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.sizing_optimization_strategy = Some(v.into());
        self
    }
    #[doc = "Set the field `target_product`.\nTarget product for assets using this preference set. Specify either target product or business goal, but not both. Possible values: 'COMPUTE_MIGRATION_TARGET_PRODUCT_UNSPECIFIED', 'COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_VMWARE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_SOLE_TENANCY'"]
    pub fn set_target_product(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.target_product = Some(v.into());
        self
    }
    #[doc = "Set the field `compute_engine_preferences`.\n"]
    pub fn set_compute_engine_preferences(
        mut self,
        v: impl Into<
            BlockAssignable<
                MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.compute_engine_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.compute_engine_preferences = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `region_preferences`.\n"]
    pub fn set_region_preferences(
        mut self,
        v: impl Into<
            BlockAssignable<
                MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.region_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.region_preferences = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `sole_tenancy_preferences`.\n"]
    pub fn set_sole_tenancy_preferences(
        mut self,
        v: impl Into<
            BlockAssignable<
                MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.sole_tenancy_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.sole_tenancy_preferences = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `vmware_engine_preferences`.\n"]
    pub fn set_vmware_engine_preferences(
        mut self,
        v: impl Into<
            BlockAssignable<
                MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.vmware_engine_preferences = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.vmware_engine_preferences = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for MigrationCenterPreferenceSetVirtualMachinePreferencesEl {
    type O = BlockAssignable<MigrationCenterPreferenceSetVirtualMachinePreferencesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildMigrationCenterPreferenceSetVirtualMachinePreferencesEl {}
impl BuildMigrationCenterPreferenceSetVirtualMachinePreferencesEl {
    pub fn build(self) -> MigrationCenterPreferenceSetVirtualMachinePreferencesEl {
        MigrationCenterPreferenceSetVirtualMachinePreferencesEl {
            commitment_plan: core::default::Default::default(),
            sizing_optimization_strategy: core::default::Default::default(),
            target_product: core::default::Default::default(),
            compute_engine_preferences: core::default::Default::default(),
            region_preferences: core::default::Default::default(),
            sole_tenancy_preferences: core::default::Default::default(),
            vmware_engine_preferences: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct MigrationCenterPreferenceSetVirtualMachinePreferencesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for MigrationCenterPreferenceSetVirtualMachinePreferencesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> MigrationCenterPreferenceSetVirtualMachinePreferencesElRef {
        MigrationCenterPreferenceSetVirtualMachinePreferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl MigrationCenterPreferenceSetVirtualMachinePreferencesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `commitment_plan` after provisioning.\nCommitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: 'COMMITMENT_PLAN_UNSPECIFIED', 'COMMITMENT_PLAN_NONE', 'COMMITMENT_PLAN_ONE_YEAR', 'COMMITMENT_PLAN_THREE_YEARS'"]
    pub fn commitment_plan(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.commitment_plan", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sizing_optimization_strategy` after provisioning.\nSizing optimization strategy specifies the preferred strategy used when extrapolating usage data to calculate insights and recommendations for a virtual machine. If you are unsure which value to set, a moderate sizing optimization strategy is often a good value to start with. Possible values: 'SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED', 'SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE', 'SIZING_OPTIMIZATION_STRATEGY_MODERATE', 'SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE'"]
    pub fn sizing_optimization_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.sizing_optimization_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `target_product` after provisioning.\nTarget product for assets using this preference set. Specify either target product or business goal, but not both. Possible values: 'COMPUTE_MIGRATION_TARGET_PRODUCT_UNSPECIFIED', 'COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_VMWARE_ENGINE', 'COMPUTE_MIGRATION_TARGET_PRODUCT_SOLE_TENANCY'"]
    pub fn target_product(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_product", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `compute_engine_preferences` after provisioning.\n"]
    pub fn compute_engine_preferences(
        &self,
    ) -> ListRef<MigrationCenterPreferenceSetVirtualMachinePreferencesElComputeEnginePreferencesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.compute_engine_preferences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `region_preferences` after provisioning.\n"]
    pub fn region_preferences(
        &self,
    ) -> ListRef<MigrationCenterPreferenceSetVirtualMachinePreferencesElRegionPreferencesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.region_preferences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `sole_tenancy_preferences` after provisioning.\n"]
    pub fn sole_tenancy_preferences(
        &self,
    ) -> ListRef<MigrationCenterPreferenceSetVirtualMachinePreferencesElSoleTenancyPreferencesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.sole_tenancy_preferences", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `vmware_engine_preferences` after provisioning.\n"]
    pub fn vmware_engine_preferences(
        &self,
    ) -> ListRef<MigrationCenterPreferenceSetVirtualMachinePreferencesElVmwareEnginePreferencesElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.vmware_engine_preferences", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct MigrationCenterPreferenceSetDynamic {
    virtual_machine_preferences:
        Option<DynamicBlock<MigrationCenterPreferenceSetVirtualMachinePreferencesEl>>,
}
