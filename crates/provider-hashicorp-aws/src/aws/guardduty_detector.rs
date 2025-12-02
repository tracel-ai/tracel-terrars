use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct GuarddutyDetectorData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finding_publishing_frequency: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datasources: Option<Vec<GuarddutyDetectorDatasourcesEl>>,
    dynamic: GuarddutyDetectorDynamic,
}
struct GuarddutyDetector_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GuarddutyDetectorData>,
}
#[derive(Clone)]
pub struct GuarddutyDetector(Rc<GuarddutyDetector_>);
impl GuarddutyDetector {
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
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable = Some(v.into());
        self
    }
    #[doc = "Set the field `finding_publishing_frequency`.\n"]
    pub fn set_finding_publishing_frequency(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().finding_publishing_frequency = Some(v.into());
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
    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }
    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }
    #[doc = "Set the field `datasources`.\n"]
    pub fn set_datasources(
        self,
        v: impl Into<BlockAssignable<GuarddutyDetectorDatasourcesEl>>,
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
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `finding_publishing_frequency` after provisioning.\n"]
    pub fn finding_publishing_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.finding_publishing_frequency", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> ListRef<GuarddutyDetectorDatasourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datasources", self.extract_ref()),
        )
    }
}
impl Referable for GuarddutyDetector {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for GuarddutyDetector {}
impl ToListMappable for GuarddutyDetector {
    type O = ListRef<GuarddutyDetectorRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for GuarddutyDetector_ {
    fn extract_resource_type(&self) -> String {
        "aws_guardduty_detector".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildGuarddutyDetector {
    pub tf_id: String,
}
impl BuildGuarddutyDetector {
    pub fn build(self, stack: &mut Stack) -> GuarddutyDetector {
        let out = GuarddutyDetector(Rc::new(GuarddutyDetector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GuarddutyDetectorData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                enable: core::default::Default::default(),
                finding_publishing_frequency: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                datasources: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct GuarddutyDetectorRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl GuarddutyDetectorRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.account_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `finding_publishing_frequency` after provisioning.\n"]
    pub fn finding_publishing_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.finding_publishing_frequency", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> ListRef<GuarddutyDetectorDatasourcesElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.datasources", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl {
    enable: PrimField<bool>,
}
impl GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl {}
impl ToListMappable for GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl {
    type O = BlockAssignable<GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl {
    #[doc = ""]
    pub enable: PrimField<bool>,
}
impl BuildGuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl {
    pub fn build(self) -> GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl {
        GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl {
            enable: self.enable,
        }
    }
}
pub struct GuarddutyDetectorDatasourcesElKubernetesElAuditLogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorDatasourcesElKubernetesElAuditLogsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyDetectorDatasourcesElKubernetesElAuditLogsElRef {
        GuarddutyDetectorDatasourcesElKubernetesElAuditLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyDetectorDatasourcesElKubernetesElAuditLogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyDetectorDatasourcesElKubernetesElDynamic {
    audit_logs: Option<DynamicBlock<GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl>>,
}
#[derive(Serialize)]
pub struct GuarddutyDetectorDatasourcesElKubernetesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    audit_logs: Option<Vec<GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl>>,
    dynamic: GuarddutyDetectorDatasourcesElKubernetesElDynamic,
}
impl GuarddutyDetectorDatasourcesElKubernetesEl {
    #[doc = "Set the field `audit_logs`.\n"]
    pub fn set_audit_logs(
        mut self,
        v: impl Into<BlockAssignable<GuarddutyDetectorDatasourcesElKubernetesElAuditLogsEl>>,
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
impl ToListMappable for GuarddutyDetectorDatasourcesElKubernetesEl {
    type O = BlockAssignable<GuarddutyDetectorDatasourcesElKubernetesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyDetectorDatasourcesElKubernetesEl {}
impl BuildGuarddutyDetectorDatasourcesElKubernetesEl {
    pub fn build(self) -> GuarddutyDetectorDatasourcesElKubernetesEl {
        GuarddutyDetectorDatasourcesElKubernetesEl {
            audit_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GuarddutyDetectorDatasourcesElKubernetesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorDatasourcesElKubernetesElRef {
    fn new(shared: StackShared, base: String) -> GuarddutyDetectorDatasourcesElKubernetesElRef {
        GuarddutyDetectorDatasourcesElKubernetesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyDetectorDatasourcesElKubernetesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `audit_logs` after provisioning.\n"]
    pub fn audit_logs(&self) -> ListRef<GuarddutyDetectorDatasourcesElKubernetesElAuditLogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.audit_logs", self.base))
    }
}
#[derive(Serialize)]
pub struct GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl
{
    enable: PrimField<bool>,
}
impl GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {}
impl ToListMappable
    for GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl
{
    type O = BlockAssignable<
        GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl
{
    #[doc = ""]
    pub enable: PrimField<bool>,
}
impl
    BuildGuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl
{
    pub fn build(
        self,
    ) -> GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl
    {
        GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {
            enable: self.enable,
        }
    }
}
pub struct GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef { fn new (shared : StackShared , base : String) -> GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef { GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef { shared : shared , base : base . to_string () , } } }
impl GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElDynamic { ebs_volumes : Option < DynamicBlock < GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl >> , }
#[derive(Serialize)]
pub struct GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl { # [serde (skip_serializing_if = "Option::is_none")] ebs_volumes : Option < Vec < GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl > > , dynamic : GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElDynamic , }
impl GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
    #[doc = "Set the field `ebs_volumes`.\n"]
    pub fn set_ebs_volumes(
        mut self,
        v : impl Into < BlockAssignable < GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl >>,
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
impl ToListMappable
    for GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl
{
    type O = BlockAssignable<
        GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {}
impl BuildGuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
    pub fn build(
        self,
    ) -> GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
        GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
            ebs_volumes: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
        GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `ebs_volumes` after provisioning.\n"]    pub fn ebs_volumes (& self) -> ListRef < GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef >{
        ListRef::new(self.shared().clone(), format!("{}.ebs_volumes", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyDetectorDatasourcesElMalwareProtectionElDynamic {
    scan_ec2_instance_with_findings: Option<
        DynamicBlock<
            GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct GuarddutyDetectorDatasourcesElMalwareProtectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    scan_ec2_instance_with_findings:
        Option<Vec<GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl>>,
    dynamic: GuarddutyDetectorDatasourcesElMalwareProtectionElDynamic,
}
impl GuarddutyDetectorDatasourcesElMalwareProtectionEl {
    #[doc = "Set the field `scan_ec2_instance_with_findings`.\n"]
    pub fn set_scan_ec2_instance_with_findings(
        mut self,
        v: impl Into<
            BlockAssignable<
                GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl,
            >,
        >,
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
impl ToListMappable for GuarddutyDetectorDatasourcesElMalwareProtectionEl {
    type O = BlockAssignable<GuarddutyDetectorDatasourcesElMalwareProtectionEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyDetectorDatasourcesElMalwareProtectionEl {}
impl BuildGuarddutyDetectorDatasourcesElMalwareProtectionEl {
    pub fn build(self) -> GuarddutyDetectorDatasourcesElMalwareProtectionEl {
        GuarddutyDetectorDatasourcesElMalwareProtectionEl {
            scan_ec2_instance_with_findings: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GuarddutyDetectorDatasourcesElMalwareProtectionElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorDatasourcesElMalwareProtectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> GuarddutyDetectorDatasourcesElMalwareProtectionElRef {
        GuarddutyDetectorDatasourcesElMalwareProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyDetectorDatasourcesElMalwareProtectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `scan_ec2_instance_with_findings` after provisioning.\n"]
    pub fn scan_ec2_instance_with_findings(
        &self,
    ) -> ListRef<GuarddutyDetectorDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef>
    {
        ListRef::new(
            self.shared().clone(),
            format!("{}.scan_ec2_instance_with_findings", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct GuarddutyDetectorDatasourcesElS3LogsEl {
    enable: PrimField<bool>,
}
impl GuarddutyDetectorDatasourcesElS3LogsEl {}
impl ToListMappable for GuarddutyDetectorDatasourcesElS3LogsEl {
    type O = BlockAssignable<GuarddutyDetectorDatasourcesElS3LogsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyDetectorDatasourcesElS3LogsEl {
    #[doc = ""]
    pub enable: PrimField<bool>,
}
impl BuildGuarddutyDetectorDatasourcesElS3LogsEl {
    pub fn build(self) -> GuarddutyDetectorDatasourcesElS3LogsEl {
        GuarddutyDetectorDatasourcesElS3LogsEl {
            enable: self.enable,
        }
    }
}
pub struct GuarddutyDetectorDatasourcesElS3LogsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorDatasourcesElS3LogsElRef {
    fn new(shared: StackShared, base: String) -> GuarddutyDetectorDatasourcesElS3LogsElRef {
        GuarddutyDetectorDatasourcesElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyDetectorDatasourcesElS3LogsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyDetectorDatasourcesElDynamic {
    kubernetes: Option<DynamicBlock<GuarddutyDetectorDatasourcesElKubernetesEl>>,
    malware_protection: Option<DynamicBlock<GuarddutyDetectorDatasourcesElMalwareProtectionEl>>,
    s3_logs: Option<DynamicBlock<GuarddutyDetectorDatasourcesElS3LogsEl>>,
}
#[derive(Serialize)]
pub struct GuarddutyDetectorDatasourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kubernetes: Option<Vec<GuarddutyDetectorDatasourcesElKubernetesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    malware_protection: Option<Vec<GuarddutyDetectorDatasourcesElMalwareProtectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_logs: Option<Vec<GuarddutyDetectorDatasourcesElS3LogsEl>>,
    dynamic: GuarddutyDetectorDatasourcesElDynamic,
}
impl GuarddutyDetectorDatasourcesEl {
    #[doc = "Set the field `kubernetes`.\n"]
    pub fn set_kubernetes(
        mut self,
        v: impl Into<BlockAssignable<GuarddutyDetectorDatasourcesElKubernetesEl>>,
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
        v: impl Into<BlockAssignable<GuarddutyDetectorDatasourcesElMalwareProtectionEl>>,
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
        v: impl Into<BlockAssignable<GuarddutyDetectorDatasourcesElS3LogsEl>>,
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
impl ToListMappable for GuarddutyDetectorDatasourcesEl {
    type O = BlockAssignable<GuarddutyDetectorDatasourcesEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildGuarddutyDetectorDatasourcesEl {}
impl BuildGuarddutyDetectorDatasourcesEl {
    pub fn build(self) -> GuarddutyDetectorDatasourcesEl {
        GuarddutyDetectorDatasourcesEl {
            kubernetes: core::default::Default::default(),
            malware_protection: core::default::Default::default(),
            s3_logs: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct GuarddutyDetectorDatasourcesElRef {
    shared: StackShared,
    base: String,
}
impl Ref for GuarddutyDetectorDatasourcesElRef {
    fn new(shared: StackShared, base: String) -> GuarddutyDetectorDatasourcesElRef {
        GuarddutyDetectorDatasourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl GuarddutyDetectorDatasourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(&self) -> ListRef<GuarddutyDetectorDatasourcesElKubernetesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.base))
    }
    #[doc = "Get a reference to the value of field `malware_protection` after provisioning.\n"]
    pub fn malware_protection(
        &self,
    ) -> ListRef<GuarddutyDetectorDatasourcesElMalwareProtectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.malware_protection", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> ListRef<GuarddutyDetectorDatasourcesElS3LogsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}
#[derive(Serialize, Default)]
struct GuarddutyDetectorDynamic {
    datasources: Option<DynamicBlock<GuarddutyDetectorDatasourcesEl>>,
}
