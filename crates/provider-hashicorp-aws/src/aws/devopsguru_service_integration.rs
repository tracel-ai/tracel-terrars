use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct DevopsguruServiceIntegrationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_server_side_encryption: Option<Vec<DevopsguruServiceIntegrationKmsServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logs_anomaly_detection: Option<Vec<DevopsguruServiceIntegrationLogsAnomalyDetectionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ops_center: Option<Vec<DevopsguruServiceIntegrationOpsCenterEl>>,
    dynamic: DevopsguruServiceIntegrationDynamic,
}

struct DevopsguruServiceIntegration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DevopsguruServiceIntegrationData>,
}

#[derive(Clone)]
pub struct DevopsguruServiceIntegration(Rc<DevopsguruServiceIntegration_>);

impl DevopsguruServiceIntegration {
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

    #[doc = "Set the field `kms_server_side_encryption`.\n"]
    pub fn set_kms_server_side_encryption(
        self,
        v: impl Into<BlockAssignable<DevopsguruServiceIntegrationKmsServerSideEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kms_server_side_encryption = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kms_server_side_encryption = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `logs_anomaly_detection`.\n"]
    pub fn set_logs_anomaly_detection(
        self,
        v: impl Into<BlockAssignable<DevopsguruServiceIntegrationLogsAnomalyDetectionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logs_anomaly_detection = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logs_anomaly_detection = Some(d);
            }
        }
        self
    }

    #[doc = "Set the field `ops_center`.\n"]
    pub fn set_ops_center(
        self,
        v: impl Into<BlockAssignable<DevopsguruServiceIntegrationOpsCenterEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ops_center = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ops_center = Some(d);
            }
        }
        self
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

    #[doc = "Get a reference to the value of field `kms_server_side_encryption` after provisioning.\n"]
    pub fn kms_server_side_encryption(
        &self,
    ) -> ListRef<DevopsguruServiceIntegrationKmsServerSideEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_server_side_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logs_anomaly_detection` after provisioning.\n"]
    pub fn logs_anomaly_detection(
        &self,
    ) -> ListRef<DevopsguruServiceIntegrationLogsAnomalyDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logs_anomaly_detection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ops_center` after provisioning.\n"]
    pub fn ops_center(&self) -> ListRef<DevopsguruServiceIntegrationOpsCenterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ops_center", self.extract_ref()),
        )
    }
}

impl Referable for DevopsguruServiceIntegration {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}

impl Resource for DevopsguruServiceIntegration {}

impl ToListMappable for DevopsguruServiceIntegration {
    type O = ListRef<DevopsguruServiceIntegrationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DevopsguruServiceIntegration_ {
    fn extract_resource_type(&self) -> String {
        "aws_devopsguru_service_integration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruServiceIntegration {
    pub tf_id: String,
}

impl BuildDevopsguruServiceIntegration {
    pub fn build(self, stack: &mut Stack) -> DevopsguruServiceIntegration {
        let out = DevopsguruServiceIntegration(Rc::new(DevopsguruServiceIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DevopsguruServiceIntegrationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                kms_server_side_encryption: core::default::Default::default(),
                logs_anomaly_detection: core::default::Default::default(),
                ops_center: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruServiceIntegrationRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruServiceIntegrationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}

impl DevopsguruServiceIntegrationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `kms_server_side_encryption` after provisioning.\n"]
    pub fn kms_server_side_encryption(
        &self,
    ) -> ListRef<DevopsguruServiceIntegrationKmsServerSideEncryptionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_server_side_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `logs_anomaly_detection` after provisioning.\n"]
    pub fn logs_anomaly_detection(
        &self,
    ) -> ListRef<DevopsguruServiceIntegrationLogsAnomalyDetectionElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.logs_anomaly_detection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ops_center` after provisioning.\n"]
    pub fn ops_center(&self) -> ListRef<DevopsguruServiceIntegrationOpsCenterElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.ops_center", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DevopsguruServiceIntegrationKmsServerSideEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_in_status: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DevopsguruServiceIntegrationKmsServerSideEncryptionEl {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_id = Some(v.into());
        self
    }

    #[doc = "Set the field `opt_in_status`.\n"]
    pub fn set_opt_in_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opt_in_status = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DevopsguruServiceIntegrationKmsServerSideEncryptionEl {
    type O = BlockAssignable<DevopsguruServiceIntegrationKmsServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruServiceIntegrationKmsServerSideEncryptionEl {}

impl BuildDevopsguruServiceIntegrationKmsServerSideEncryptionEl {
    pub fn build(self) -> DevopsguruServiceIntegrationKmsServerSideEncryptionEl {
        DevopsguruServiceIntegrationKmsServerSideEncryptionEl {
            kms_key_id: core::default::Default::default(),
            opt_in_status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DevopsguruServiceIntegrationKmsServerSideEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruServiceIntegrationKmsServerSideEncryptionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DevopsguruServiceIntegrationKmsServerSideEncryptionElRef {
        DevopsguruServiceIntegrationKmsServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruServiceIntegrationKmsServerSideEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `opt_in_status` after provisioning.\n"]
    pub fn opt_in_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.opt_in_status", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DevopsguruServiceIntegrationLogsAnomalyDetectionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_in_status: Option<PrimField<String>>,
}

impl DevopsguruServiceIntegrationLogsAnomalyDetectionEl {
    #[doc = "Set the field `opt_in_status`.\n"]
    pub fn set_opt_in_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opt_in_status = Some(v.into());
        self
    }
}

impl ToListMappable for DevopsguruServiceIntegrationLogsAnomalyDetectionEl {
    type O = BlockAssignable<DevopsguruServiceIntegrationLogsAnomalyDetectionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruServiceIntegrationLogsAnomalyDetectionEl {}

impl BuildDevopsguruServiceIntegrationLogsAnomalyDetectionEl {
    pub fn build(self) -> DevopsguruServiceIntegrationLogsAnomalyDetectionEl {
        DevopsguruServiceIntegrationLogsAnomalyDetectionEl {
            opt_in_status: core::default::Default::default(),
        }
    }
}

pub struct DevopsguruServiceIntegrationLogsAnomalyDetectionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruServiceIntegrationLogsAnomalyDetectionElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DevopsguruServiceIntegrationLogsAnomalyDetectionElRef {
        DevopsguruServiceIntegrationLogsAnomalyDetectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruServiceIntegrationLogsAnomalyDetectionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `opt_in_status` after provisioning.\n"]
    pub fn opt_in_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.opt_in_status", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DevopsguruServiceIntegrationOpsCenterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    opt_in_status: Option<PrimField<String>>,
}

impl DevopsguruServiceIntegrationOpsCenterEl {
    #[doc = "Set the field `opt_in_status`.\n"]
    pub fn set_opt_in_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.opt_in_status = Some(v.into());
        self
    }
}

impl ToListMappable for DevopsguruServiceIntegrationOpsCenterEl {
    type O = BlockAssignable<DevopsguruServiceIntegrationOpsCenterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruServiceIntegrationOpsCenterEl {}

impl BuildDevopsguruServiceIntegrationOpsCenterEl {
    pub fn build(self) -> DevopsguruServiceIntegrationOpsCenterEl {
        DevopsguruServiceIntegrationOpsCenterEl {
            opt_in_status: core::default::Default::default(),
        }
    }
}

pub struct DevopsguruServiceIntegrationOpsCenterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DevopsguruServiceIntegrationOpsCenterElRef {
    fn new(shared: StackShared, base: String) -> DevopsguruServiceIntegrationOpsCenterElRef {
        DevopsguruServiceIntegrationOpsCenterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruServiceIntegrationOpsCenterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `opt_in_status` after provisioning.\n"]
    pub fn opt_in_status(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.opt_in_status", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct DevopsguruServiceIntegrationDynamic {
    kms_server_side_encryption:
        Option<DynamicBlock<DevopsguruServiceIntegrationKmsServerSideEncryptionEl>>,
    logs_anomaly_detection:
        Option<DynamicBlock<DevopsguruServiceIntegrationLogsAnomalyDetectionEl>>,
    ops_center: Option<DynamicBlock<DevopsguruServiceIntegrationOpsCenterEl>>,
}
