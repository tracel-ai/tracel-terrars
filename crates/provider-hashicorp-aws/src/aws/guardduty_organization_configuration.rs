use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GuarddutyOrganizationConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    auto_enable_organization_members: PrimField<String>,
    detector_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datasources: Option<Vec<GuarddutyOrganizationConfigurationDatasourcesEl>>,
    dynamic: GuarddutyOrganizationConfigurationDynamic,
}
struct GuarddutyOrganizationConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GuarddutyOrganizationConfigurationData>,
}
#[derive(Clone)]
pub struct GuarddutyOrganizationConfiguration(Rc<GuarddutyOrganizationConfiguration_>);
impl GuarddutyOrganizationConfiguration {
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
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }
    #[doc = "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }
    #[doc = "Set the field `datasources`.\n"]
    pub fn set_datasources(
        self,
        v: impl Into<BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().datasources = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.datasources = Some(d);
            }
        }
        self
    }
    #[doc = "Get a reference to the value of field `auto_enable_organization_members` after provisioning.\n"]
    pub fn auto_enable_organization_members(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_enable_organization_members", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> ListRef<GuarddutyOrganizationConfigurationDatasourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datasources", self.extract_ref()),
        )
    }
}
impl Referable for GuarddutyOrganizationConfiguration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GuarddutyOrganizationConfiguration {}
impl ToListMappable for GuarddutyOrganizationConfiguration {
    type O = ListRef<GuarddutyOrganizationConfigurationRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GuarddutyOrganizationConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_guardduty_organization_configuration".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGuarddutyOrganizationConfiguration {
    pub tf_id: String,
    #[doc = ""]
    pub auto_enable_organization_members: PrimField<String>,
    #[doc = ""]
    pub detector_id: PrimField<String>,
}
impl BuildGuarddutyOrganizationConfiguration {
    pub fn build(self, stack: &mut Stack) -> GuarddutyOrganizationConfiguration {
        let out =
            GuarddutyOrganizationConfiguration(Rc::new(GuarddutyOrganizationConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(GuarddutyOrganizationConfigurationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    auto_enable_organization_members: self.auto_enable_organization_members,
                    detector_id: self.detector_id,
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    datasources: core::default::Default::default(),
                    dynamic: Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GuarddutyOrganizationConfigurationRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GuarddutyOrganizationConfigurationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_enable_organization_members` after provisioning.\n"]
    pub fn auto_enable_organization_members(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_enable_organization_members", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.detector_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.region", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> ListRef<GuarddutyOrganizationConfigurationDatasourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datasources", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    enable: PrimField<bool>,
}
impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {}
impl ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    type O =
        BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    #[doc = ""]
    pub enable: PrimField<bool>,
}
impl BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
            enable: self.enable,
        }
    }
}
pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElDynamic {
    audit_logs: Option<
        DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>,
    >,
}
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_logs: Option<Vec<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>>,
    dynamic: GuarddutyOrganizationConfigurationDatasourcesElKubernetesElDynamic,
}
impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    #[doc = "Set the field `audit_logs`.\n"]
    pub fn set_audit_logs(
        mut self,
        v: impl Into<
            BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.audit_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_logs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    type O = BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {}
impl BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
            audit_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit_logs` after provisioning.\n"]
    pub fn audit_logs(
        &self,
    ) -> ListRef<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audit_logs", self.base))
    }
}
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl
{
    auto_enable: PrimField<bool>,
}
impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl { }
impl ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl { type O = BlockAssignable < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl
{
    #[doc = ""]
    pub auto_enable: PrimField<bool>,
}
impl BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl { pub fn build (self) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl { GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl { auto_enable : self . auto_enable , } } }
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef { fn new (shared : StackShared , base : String) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef { GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef { shared : shared , base : base . to_string () , } } }
impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"] pub fn auto_enable (& self) -> PrimExpr < bool > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.auto_enable" , self . base)) } }
#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElDynamic { ebs_volumes : Option < DynamicBlock < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl >> , }
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl { # [serde (skip_serializing_if = "Option::is_none")] ebs_volumes : Option < Vec < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl > > , dynamic : GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElDynamic , }
impl
    GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl
{
    #[doc = "Set the field `ebs_volumes`.\n"]
    pub fn set_ebs_volumes(
        mut self,
        v : impl Into < BlockAssignable < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.ebs_volumes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_volumes = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl { type O = BlockAssignable < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl
{}
impl BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl { pub fn build (self) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl { GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl { ebs_volumes : core :: default :: Default :: default () , dynamic : Default :: default () , } } }
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef { fn new (shared : StackShared , base : String) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef { GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef { shared : shared , base : base . to_string () , } } }
impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `ebs_volumes` after provisioning.\n"] pub fn ebs_volumes (& self) -> ListRef < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef > { ListRef :: new (self . shared () . clone () , format ! ("{}.ebs_volumes" , self . base)) } }
#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElDynamic { scan_ec2_instance_with_findings : Option < DynamicBlock < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl >> , }
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl { # [serde (skip_serializing_if = "Option::is_none")] scan_ec2_instance_with_findings : Option < Vec < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl > > , dynamic : GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElDynamic , }
impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
    #[doc = "Set the field `scan_ec2_instance_with_findings`.\n"]
    pub fn set_scan_ec2_instance_with_findings(
        mut self,
        v : impl Into < BlockAssignable < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.scan_ec2_instance_with_findings = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.scan_ec2_instance_with_findings = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
    type O = BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {}
impl BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
            scan_ec2_instance_with_findings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scan_ec2_instance_with_findings` after provisioning.\n"]    pub fn scan_ec2_instance_with_findings (& self) -> ListRef < GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.scan_ec2_instance_with_findings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    auto_enable: PrimField<bool>,
}
impl GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {}
impl ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    type O = BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    #[doc = ""]
    pub auto_enable: PrimField<bool>,
}
impl BuildGuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
        GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
            auto_enable: self.auto_enable,
        }
    }
}
pub struct GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
        GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElDynamic {
    kubernetes: Option<DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>>,
    malware_protection:
        Option<DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>>,
    s3_logs: Option<DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>>,
}
#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes: Option<Vec<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    malware_protection:
        Option<Vec<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_logs: Option<Vec<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>>,
    dynamic: GuarddutyOrganizationConfigurationDatasourcesElDynamic,
}
impl GuarddutyOrganizationConfigurationDatasourcesEl {
    #[doc = "Set the field `kubernetes`.\n"]
    pub fn set_kubernetes(
        mut self,
        v: impl Into<BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kubernetes = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `malware_protection`.\n"]
    pub fn set_malware_protection(
        mut self,
        v: impl Into<
            BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.malware_protection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_protection = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `s3_logs`.\n"]
    pub fn set_s3_logs(
        mut self,
        v: impl Into<BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.s3_logs = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_logs = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for GuarddutyOrganizationConfigurationDatasourcesEl {
    type O = BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyOrganizationConfigurationDatasourcesEl {}
impl BuildGuarddutyOrganizationConfigurationDatasourcesEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesEl {
        GuarddutyOrganizationConfigurationDatasourcesEl {
            kubernetes: core::default::Default::default(),
            malware_protection: core::default::Default::default(),
            s3_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GuarddutyOrganizationConfigurationDatasourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyOrganizationConfigurationDatasourcesElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElRef {
        GuarddutyOrganizationConfigurationDatasourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyOrganizationConfigurationDatasourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(
        &self,
    ) -> ListRef<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.base))
    }
    #[doc = "Get a reference to the value of field `malware_protection` after provisioning.\n"]
    pub fn malware_protection(
        &self,
    ) -> ListRef<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.malware_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> ListRef<GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDynamic {
    datasources: Option<DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesEl>>,
}
