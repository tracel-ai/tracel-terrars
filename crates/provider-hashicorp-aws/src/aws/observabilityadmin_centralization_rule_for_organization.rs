use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct ObservabilityadminCentralizationRuleForOrganizationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    rule_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rule: Option<Vec<ObservabilityadminCentralizationRuleForOrganizationRuleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl>,
    dynamic: ObservabilityadminCentralizationRuleForOrganizationDynamic,
}
struct ObservabilityadminCentralizationRuleForOrganization_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ObservabilityadminCentralizationRuleForOrganizationData>,
}
#[derive(Clone)]
pub struct ObservabilityadminCentralizationRuleForOrganization(
    Rc<ObservabilityadminCentralizationRuleForOrganization_>,
);
impl ObservabilityadminCentralizationRuleForOrganization {
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
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl Into<BlockAssignable<ObservabilityadminCentralizationRuleForOrganizationRuleEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl Into<ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_arn` after provisioning.\n"]
    pub fn rule_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<ObservabilityadminCentralizationRuleForOrganizationRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef {
        ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for ObservabilityadminCentralizationRuleForOrganization {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for ObservabilityadminCentralizationRuleForOrganization {}
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganization {
    type O = ListRef<ObservabilityadminCentralizationRuleForOrganizationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for ObservabilityadminCentralizationRuleForOrganization_ {
    fn extract_resource_type(&self) -> String {
        "aws_observabilityadmin_centralization_rule_for_organization".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildObservabilityadminCentralizationRuleForOrganization {
    pub tf_id: String,
    #[doc = ""]
    pub rule_name: PrimField<String>,
}
impl BuildObservabilityadminCentralizationRuleForOrganization {
    pub fn build(self, stack: &mut Stack) -> ObservabilityadminCentralizationRuleForOrganization {
        let out = ObservabilityadminCentralizationRuleForOrganization(Rc::new(
            ObservabilityadminCentralizationRuleForOrganization_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(ObservabilityadminCentralizationRuleForOrganizationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    region: core::default::Default::default(),
                    rule_name: self.rule_name,
                    tags: core::default::Default::default(),
                    rule: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            },
        ));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct ObservabilityadminCentralizationRuleForOrganizationRef {
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl ObservabilityadminCentralizationRuleForOrganizationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_arn` after provisioning.\n"]
    pub fn rule_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_arn", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.rule_name", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(
            self.shared().clone(),
            format!("{}.tags_all", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> ListRef<ObservabilityadminCentralizationRuleForOrganizationRuleElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.rule", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef {
        ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl { # [doc = "Set the field `kms_key_arn`.\n"] pub fn set_kms_key_arn (mut self , v : impl Into < PrimField < String > >) -> Self { self . kms_key_arn = Some (v . into ()) ; self } # [doc = "Set the field `region`.\n"] pub fn set_region (mut self , v : impl Into < PrimField < String > >) -> Self { self . region = Some (v . into ()) ; self } }
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl { type O = BlockAssignable < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl
{}
impl BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl { pub fn build (self) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl { ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl { kms_key_arn : core :: default :: Default :: default () , region : core :: default :: Default :: default () , } } }
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationElRef { fn new (shared : StackShared , base : String) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationElRef { ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"] pub fn kms_key_arn (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.kms_key_arn" , self . base)) } # [doc = "Get a reference to the value of field `region` after provisioning.\n"] pub fn region (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.region" , self . base)) } }
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl
{
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_conflict_resolution_strategy: Option<PrimField<String>>,
    encryption_strategy: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
}
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl { # [doc = "Set the field `encryption_conflict_resolution_strategy`.\n"] pub fn set_encryption_conflict_resolution_strategy (mut self , v : impl Into < PrimField < String > >) -> Self { self . encryption_conflict_resolution_strategy = Some (v . into ()) ; self } # [doc = "Set the field `kms_key_arn`.\n"] pub fn set_kms_key_arn (mut self , v : impl Into < PrimField < String > >) -> Self { self . kms_key_arn = Some (v . into ()) ; self } }
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl { type O = BlockAssignable < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl
{
    #[doc = ""]
    pub encryption_strategy: PrimField<String>,
}
impl BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl { pub fn build (self) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl { ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl { encryption_conflict_resolution_strategy : core :: default :: Default :: default () , encryption_strategy : self . encryption_strategy , kms_key_arn : core :: default :: Default :: default () , } } }
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationElRef { fn new (shared : StackShared , base : String) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationElRef { ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `encryption_conflict_resolution_strategy` after provisioning.\n"] pub fn encryption_conflict_resolution_strategy (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.encryption_conflict_resolution_strategy" , self . base)) } # [doc = "Get a reference to the value of field `encryption_strategy` after provisioning.\n"] pub fn encryption_strategy (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.encryption_strategy" , self . base)) } # [doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"] pub fn kms_key_arn (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.kms_key_arn" , self . base)) } }
#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElDynamic { backup_configuration : Option < DynamicBlock < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl >> , logs_encryption_configuration : Option < DynamicBlock < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl >> , }
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl { # [serde (skip_serializing_if = "Option::is_none")] backup_configuration : Option < Vec < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl > > , # [serde (skip_serializing_if = "Option::is_none")] logs_encryption_configuration : Option < Vec < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl > > , dynamic : ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElDynamic , }
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl { # [doc = "Set the field `backup_configuration`.\n"] pub fn set_backup_configuration (mut self , v : impl Into < BlockAssignable < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . backup_configuration = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . backup_configuration = Some (d) ; } } self } # [doc = "Set the field `logs_encryption_configuration`.\n"] pub fn set_logs_encryption_configuration (mut self , v : impl Into < BlockAssignable < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationEl >>) -> Self { match v . into () { BlockAssignable :: Literal (v) => { self . logs_encryption_configuration = Some (v) ; } , BlockAssignable :: Dynamic (d) => { self . dynamic . logs_encryption_configuration = Some (d) ; } } self } }
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl { type O = BlockAssignable < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl
{}
impl BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl { pub fn build (self) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl { ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl { backup_configuration : core :: default :: Default :: default () , logs_encryption_configuration : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElRef { fn new (shared : StackShared , base : String) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElRef { ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"] pub fn backup_configuration (& self) -> ListRef < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElBackupConfigurationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.backup_configuration" , self . base)) } # [doc = "Get a reference to the value of field `logs_encryption_configuration` after provisioning.\n"] pub fn logs_encryption_configuration (& self) -> ListRef < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElLogsEncryptionConfigurationElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.logs_encryption_configuration" , self . base)) } }
#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDynamic { destination_logs_configuration : Option < DynamicBlock < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl >> , }
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl { account : PrimField < String > , region : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] destination_logs_configuration : Option < Vec < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl > > , dynamic : ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDynamic , }
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl {
    #[doc = "Set the field `destination_logs_configuration`.\n"]
    pub fn set_destination_logs_configuration(
        mut self,
        v : impl Into < BlockAssignable < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination_logs_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_logs_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl {
    type O =
        BlockAssignable<ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl {
    #[doc = ""]
    pub account: PrimField<String>,
    #[doc = ""]
    pub region: PrimField<String>,
}
impl BuildObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl {
        ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl {
            account: self.account,
            region: self.region,
            destination_logs_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElRef {
        ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
    #[doc = "Get a reference to the value of field `destination_logs_configuration` after provisioning.\n"]    pub fn destination_logs_configuration (& self) -> ListRef < ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElDestinationLogsConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.destination_logs_configuration", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl
{
    encrypted_log_group_strategy: PrimField<String>,
    log_group_selection_criteria: PrimField<String>,
}
impl ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl {}
impl ToListMappable
    for ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl
{
    type O = BlockAssignable<
        ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl
{
    #[doc = ""]
    pub encrypted_log_group_strategy: PrimField<String>,
    #[doc = ""]
    pub log_group_selection_criteria: PrimField<String>,
}
impl
    BuildObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl
{
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl
    {
        ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl {
            encrypted_log_group_strategy: self.encrypted_log_group_strategy,
            log_group_selection_criteria: self.log_group_selection_criteria,
        }
    }
}
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationElRef { fn new (shared : StackShared , base : String) -> ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationElRef { ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationElRef { shared : shared , base : base . to_string () , } } }
impl ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `encrypted_log_group_strategy` after provisioning.\n"]
    pub fn encrypted_log_group_strategy(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.encrypted_log_group_strategy", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `log_group_selection_criteria` after provisioning.\n"]
    pub fn log_group_selection_criteria(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.log_group_selection_criteria", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElDynamic { source_logs_configuration : Option < DynamicBlock < ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl >> , }
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl { regions : SetField < PrimField < String > > , scope : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] source_logs_configuration : Option < Vec < ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl > > , dynamic : ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElDynamic , }
impl ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl {
    #[doc = "Set the field `source_logs_configuration`.\n"]
    pub fn set_source_logs_configuration(
        mut self,
        v : impl Into < BlockAssignable < ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_logs_configuration = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_logs_configuration = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl {
    type O = BlockAssignable<ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl {
    #[doc = ""]
    pub regions: SetField<PrimField<String>>,
    #[doc = ""]
    pub scope: PrimField<String>,
}
impl BuildObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl {
        ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl {
            regions: self.regions,
            scope: self.scope,
            source_logs_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElRef {
        ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
    #[doc = "Get a reference to the value of field `source_logs_configuration` after provisioning.\n"]    pub fn source_logs_configuration (& self) -> ListRef < ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElSourceLogsConfigurationElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.source_logs_configuration", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganizationRuleElDynamic {
    destination: Option<
        DynamicBlock<ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl>,
    >,
    source: Option<DynamicBlock<ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl>>,
}
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    destination:
        Option<Vec<ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Vec<ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl>>,
    dynamic: ObservabilityadminCentralizationRuleForOrganizationRuleElDynamic,
}
impl ObservabilityadminCentralizationRuleForOrganizationRuleEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl Into<
            BlockAssignable<ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.destination = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v: impl Into<BlockAssignable<ObservabilityadminCentralizationRuleForOrganizationRuleElSourceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganizationRuleEl {
    type O = BlockAssignable<ObservabilityadminCentralizationRuleForOrganizationRuleEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildObservabilityadminCentralizationRuleForOrganizationRuleEl {}
impl BuildObservabilityadminCentralizationRuleForOrganizationRuleEl {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganizationRuleEl {
        ObservabilityadminCentralizationRuleForOrganizationRuleEl {
            destination: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct ObservabilityadminCentralizationRuleForOrganizationRuleElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationRuleElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ObservabilityadminCentralizationRuleForOrganizationRuleElRef {
        ObservabilityadminCentralizationRuleForOrganizationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ObservabilityadminCentralizationRuleForOrganizationRuleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> ListRef<ObservabilityadminCentralizationRuleForOrganizationRuleElDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }
    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(
        &self,
    ) -> ListRef<ObservabilityadminCentralizationRuleForOrganizationRuleElSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}
#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    update: Option<PrimField<String>>,
}
impl ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
    #[doc = "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.update = Some(v.into());
        self
    }
}
impl ToListMappable for ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl {
    type O = BlockAssignable<ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildObservabilityadminCentralizationRuleForOrganizationTimeoutsEl {}
impl BuildObservabilityadminCentralizationRuleForOrganizationTimeoutsEl {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl {
        ObservabilityadminCentralizationRuleForOrganizationTimeoutsEl {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}
pub struct ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef {
        ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl ObservabilityadminCentralizationRuleForOrganizationTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
    #[doc = "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganizationDynamic {
    rule: Option<DynamicBlock<ObservabilityadminCentralizationRuleForOrganizationRuleEl>>,
}
