use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataexchangeEventActionData {
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
    action: Option<Vec<DataexchangeEventActionActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event: Option<Vec<DataexchangeEventActionEventEl>>,
    dynamic: DataexchangeEventActionDynamic,
}

struct DataexchangeEventAction_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataexchangeEventActionData>,
}

#[derive(Clone)]
pub struct DataexchangeEventAction(Rc<DataexchangeEventAction_>);

impl DataexchangeEventAction {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(self, v: impl Into<BlockAssignable<DataexchangeEventActionActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event`.\n"]
    pub fn set_event(self, v: impl Into<BlockAssignable<DataexchangeEventActionEventEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().event = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.event = Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataexchangeEventActionActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> ListRef<DataexchangeEventActionEventElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event", self.extract_ref()))
    }
}

impl Referable for DataexchangeEventAction {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for DataexchangeEventAction { }

impl ToListMappable for DataexchangeEventAction {
    type O = ListRef<DataexchangeEventActionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for DataexchangeEventAction_ {
    fn extract_resource_type(&self) -> String {
        "aws_dataexchange_event_action".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataexchangeEventAction {
    pub tf_id: String,
}

impl BuildDataexchangeEventAction {
    pub fn build(self, stack: &mut Stack) -> DataexchangeEventAction {
        let out = DataexchangeEventAction(Rc::new(DataexchangeEventAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataexchangeEventActionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                region: core::default::Default::default(),
                action: core::default::Default::default(),
                event: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataexchangeEventActionRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataexchangeEventActionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl DataexchangeEventActionRef {
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

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<DataexchangeEventActionActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> ListRef<DataexchangeEventActionEventElRef> {
        ListRef::new(self.shared().clone(), format!("{}.event", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    kms_key_arn: Option<PrimField<String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.kms_key_arn = Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl {
    type O = BlockAssignable<DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl {}

impl BuildDataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl {
    pub fn build(self) -> DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl {
        DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl {
            kms_key_arn: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataexchangeEventActionActionElExportRevisionToS3ElEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataexchangeEventActionActionElExportRevisionToS3ElEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataexchangeEventActionActionElExportRevisionToS3ElEncryptionElRef {
        DataexchangeEventActionActionElExportRevisionToS3ElEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventActionActionElExportRevisionToS3ElEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl {
    bucket: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_pattern: Option<PrimField<String>>,
}

impl DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl {
    #[doc = "Set the field `key_pattern`.\n"]
    pub fn set_key_pattern(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_pattern = Some(v.into());
        self
    }
}

impl ToListMappable for DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl {
    type O = BlockAssignable<DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}

impl BuildDataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl {
    pub fn build(self) -> DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl {
        DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl {
            bucket: self.bucket,
            key_pattern: core::default::Default::default(),
        }
    }
}

pub struct DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationElRef {
        DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key_pattern` after provisioning.\n"]
    pub fn key_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_pattern", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventActionActionElExportRevisionToS3ElDynamic {
    encryption: Option<DynamicBlock<DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl>>,
    revision_destination: Option<
        DynamicBlock<DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl>,
    >,
}

#[derive(Serialize)]
pub struct DataexchangeEventActionActionElExportRevisionToS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption: Option<Vec<DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    revision_destination: Option<Vec<DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl>>,
    dynamic: DataexchangeEventActionActionElExportRevisionToS3ElDynamic,
}

impl DataexchangeEventActionActionElExportRevisionToS3El {
    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        mut self,
        v: impl Into<BlockAssignable<DataexchangeEventActionActionElExportRevisionToS3ElEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.encryption = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption = Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `revision_destination`.\n"]
    pub fn set_revision_destination(
        mut self,
        v: impl Into<BlockAssignable<DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.revision_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.revision_destination = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataexchangeEventActionActionElExportRevisionToS3El {
    type O = BlockAssignable<DataexchangeEventActionActionElExportRevisionToS3El>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventActionActionElExportRevisionToS3El {}

impl BuildDataexchangeEventActionActionElExportRevisionToS3El {
    pub fn build(self) -> DataexchangeEventActionActionElExportRevisionToS3El {
        DataexchangeEventActionActionElExportRevisionToS3El {
            encryption: core::default::Default::default(),
            revision_destination: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataexchangeEventActionActionElExportRevisionToS3ElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataexchangeEventActionActionElExportRevisionToS3ElRef {
    fn new(shared: StackShared, base: String) -> DataexchangeEventActionActionElExportRevisionToS3ElRef {
        DataexchangeEventActionActionElExportRevisionToS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventActionActionElExportRevisionToS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(&self) -> ListRef<DataexchangeEventActionActionElExportRevisionToS3ElEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `revision_destination` after provisioning.\n"]
    pub fn revision_destination(
        &self,
    ) -> ListRef<DataexchangeEventActionActionElExportRevisionToS3ElRevisionDestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revision_destination", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventActionActionElDynamic {
    export_revision_to_s3: Option<DynamicBlock<DataexchangeEventActionActionElExportRevisionToS3El>>,
}

#[derive(Serialize)]
pub struct DataexchangeEventActionActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    export_revision_to_s3: Option<Vec<DataexchangeEventActionActionElExportRevisionToS3El>>,
    dynamic: DataexchangeEventActionActionElDynamic,
}

impl DataexchangeEventActionActionEl {
    #[doc = "Set the field `export_revision_to_s3`.\n"]
    pub fn set_export_revision_to_s3(
        mut self,
        v: impl Into<BlockAssignable<DataexchangeEventActionActionElExportRevisionToS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.export_revision_to_s3 = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.export_revision_to_s3 = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataexchangeEventActionActionEl {
    type O = BlockAssignable<DataexchangeEventActionActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventActionActionEl {}

impl BuildDataexchangeEventActionActionEl {
    pub fn build(self) -> DataexchangeEventActionActionEl {
        DataexchangeEventActionActionEl {
            export_revision_to_s3: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataexchangeEventActionActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataexchangeEventActionActionElRef {
    fn new(shared: StackShared, base: String) -> DataexchangeEventActionActionElRef {
        DataexchangeEventActionActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventActionActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `export_revision_to_s3` after provisioning.\n"]
    pub fn export_revision_to_s3(&self) -> ListRef<DataexchangeEventActionActionElExportRevisionToS3ElRef> {
        ListRef::new(self.shared().clone(), format!("{}.export_revision_to_s3", self.base))
    }
}

#[derive(Serialize)]
pub struct DataexchangeEventActionEventElRevisionPublishedEl {
    data_set_id: PrimField<String>,
}

impl DataexchangeEventActionEventElRevisionPublishedEl { }

impl ToListMappable for DataexchangeEventActionEventElRevisionPublishedEl {
    type O = BlockAssignable<DataexchangeEventActionEventElRevisionPublishedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventActionEventElRevisionPublishedEl {
    #[doc = ""]
    pub data_set_id: PrimField<String>,
}

impl BuildDataexchangeEventActionEventElRevisionPublishedEl {
    pub fn build(self) -> DataexchangeEventActionEventElRevisionPublishedEl {
        DataexchangeEventActionEventElRevisionPublishedEl { data_set_id: self.data_set_id }
    }
}

pub struct DataexchangeEventActionEventElRevisionPublishedElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataexchangeEventActionEventElRevisionPublishedElRef {
    fn new(shared: StackShared, base: String) -> DataexchangeEventActionEventElRevisionPublishedElRef {
        DataexchangeEventActionEventElRevisionPublishedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventActionEventElRevisionPublishedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventActionEventElDynamic {
    revision_published: Option<DynamicBlock<DataexchangeEventActionEventElRevisionPublishedEl>>,
}

#[derive(Serialize)]
pub struct DataexchangeEventActionEventEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    revision_published: Option<Vec<DataexchangeEventActionEventElRevisionPublishedEl>>,
    dynamic: DataexchangeEventActionEventElDynamic,
}

impl DataexchangeEventActionEventEl {
    #[doc = "Set the field `revision_published`.\n"]
    pub fn set_revision_published(
        mut self,
        v: impl Into<BlockAssignable<DataexchangeEventActionEventElRevisionPublishedEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.revision_published = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.revision_published = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataexchangeEventActionEventEl {
    type O = BlockAssignable<DataexchangeEventActionEventEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeEventActionEventEl {}

impl BuildDataexchangeEventActionEventEl {
    pub fn build(self) -> DataexchangeEventActionEventEl {
        DataexchangeEventActionEventEl {
            revision_published: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct DataexchangeEventActionEventElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataexchangeEventActionEventElRef {
    fn new(shared: StackShared, base: String) -> DataexchangeEventActionEventElRef {
        DataexchangeEventActionEventElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeEventActionEventElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `revision_published` after provisioning.\n"]
    pub fn revision_published(&self) -> ListRef<DataexchangeEventActionEventElRevisionPublishedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.revision_published", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeEventActionDynamic {
    action: Option<DynamicBlock<DataexchangeEventActionActionEl>>,
    event: Option<DynamicBlock<DataexchangeEventActionEventEl>>,
}
