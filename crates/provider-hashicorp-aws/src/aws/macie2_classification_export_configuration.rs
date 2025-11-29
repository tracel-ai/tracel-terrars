use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Macie2ClassificationExportConfigurationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_destination: Option<Vec<Macie2ClassificationExportConfigurationS3DestinationEl>>,
    dynamic: Macie2ClassificationExportConfigurationDynamic,
}

struct Macie2ClassificationExportConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Macie2ClassificationExportConfigurationData>,
}

#[derive(Clone)]
pub struct Macie2ClassificationExportConfiguration(Rc<Macie2ClassificationExportConfiguration_>);

impl Macie2ClassificationExportConfiguration {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().region = Some(v.into());
        self
    }

    #[doc = "Set the field `s3_destination`.\n"]
    pub fn set_s3_destination(
        self,
        v: impl Into<BlockAssignable<Macie2ClassificationExportConfigurationS3DestinationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_destination = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_destination = Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> ListRef<Macie2ClassificationExportConfigurationS3DestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.extract_ref()))
    }
}

impl Referable for Macie2ClassificationExportConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Macie2ClassificationExportConfiguration { }

impl ToListMappable for Macie2ClassificationExportConfiguration {
    type O = ListRef<Macie2ClassificationExportConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Macie2ClassificationExportConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_macie2_classification_export_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMacie2ClassificationExportConfiguration {
    pub tf_id: String,
}

impl BuildMacie2ClassificationExportConfiguration {
    pub fn build(self, stack: &mut Stack) -> Macie2ClassificationExportConfiguration {
        let out = Macie2ClassificationExportConfiguration(Rc::new(Macie2ClassificationExportConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Macie2ClassificationExportConfigurationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                s3_destination: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Macie2ClassificationExportConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for Macie2ClassificationExportConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared,
            base,
        }
    }
}

impl Macie2ClassificationExportConfigurationRef {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_destination` after provisioning.\n"]
    pub fn s3_destination(&self) -> ListRef<Macie2ClassificationExportConfigurationS3DestinationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.s3_destination", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Macie2ClassificationExportConfigurationS3DestinationEl {
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key_prefix: Option<PrimField<String>>,
    kms_key_arn: PrimField<String>,
}

impl Macie2ClassificationExportConfigurationS3DestinationEl {
    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for Macie2ClassificationExportConfigurationS3DestinationEl {
    type O = BlockAssignable<Macie2ClassificationExportConfigurationS3DestinationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMacie2ClassificationExportConfigurationS3DestinationEl {
    #[doc = ""]
    pub bucket_name: PrimField<String>,
    #[doc = ""]
    pub kms_key_arn: PrimField<String>,
}

impl BuildMacie2ClassificationExportConfigurationS3DestinationEl {
    pub fn build(self) -> Macie2ClassificationExportConfigurationS3DestinationEl {
        Macie2ClassificationExportConfigurationS3DestinationEl {
            bucket_name: self.bucket_name,
            key_prefix: core::default::Default::default(),
            kms_key_arn: self.kms_key_arn,
        }
    }
}

pub struct Macie2ClassificationExportConfigurationS3DestinationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Macie2ClassificationExportConfigurationS3DestinationElRef {
    fn new(shared: StackShared, base: String) -> Macie2ClassificationExportConfigurationS3DestinationElRef {
        Macie2ClassificationExportConfigurationS3DestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Macie2ClassificationExportConfigurationS3DestinationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct Macie2ClassificationExportConfigurationDynamic {
    s3_destination: Option<DynamicBlock<Macie2ClassificationExportConfigurationS3DestinationEl>>,
}
