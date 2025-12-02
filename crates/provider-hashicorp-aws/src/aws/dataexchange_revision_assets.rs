use super::provider::ProviderAws;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
#[derive(Serialize)]
struct DataexchangeRevisionAssetsData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<PrimField<String>>,
    data_set_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finalized: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    force_destroy: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    asset: Option<Vec<DataexchangeRevisionAssetsAssetEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataexchangeRevisionAssetsTimeoutsEl>,
    dynamic: DataexchangeRevisionAssetsDynamic,
}
struct DataexchangeRevisionAssets_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataexchangeRevisionAssetsData>,
}
#[derive(Clone)]
pub struct DataexchangeRevisionAssets(Rc<DataexchangeRevisionAssets_>);
impl DataexchangeRevisionAssets {
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
    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().comment = Some(v.into());
        self
    }
    #[doc = "Set the field `finalized`.\n"]
    pub fn set_finalized(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().finalized = Some(v.into());
        self
    }
    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = Some(v.into());
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
    #[doc = "Set the field `asset`.\n"]
    pub fn set_asset(
        self,
        v: impl Into<BlockAssignable<DataexchangeRevisionAssetsAssetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().asset = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.asset = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataexchangeRevisionAssetsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }
    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `finalized` after provisioning.\n"]
    pub fn finalized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.finalized", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataexchangeRevisionAssetsTimeoutsElRef {
        DataexchangeRevisionAssetsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
impl Referable for DataexchangeRevisionAssets {
    fn extract_ref(&self) -> String {
        format!(
            "{}.{}",
            self.0.extract_resource_type(),
            self.0.extract_tf_id()
        )
    }
}
impl Resource for DataexchangeRevisionAssets {}
impl ToListMappable for DataexchangeRevisionAssets {
    type O = ListRef<DataexchangeRevisionAssetsRef>;
    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}
impl Resource_ for DataexchangeRevisionAssets_ {
    fn extract_resource_type(&self) -> String {
        "aws_dataexchange_revision_assets".into()
    }
    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }
    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}
pub struct BuildDataexchangeRevisionAssets {
    pub tf_id: String,
    #[doc = ""]
    pub data_set_id: PrimField<String>,
}
impl BuildDataexchangeRevisionAssets {
    pub fn build(self, stack: &mut Stack) -> DataexchangeRevisionAssets {
        let out = DataexchangeRevisionAssets(Rc::new(DataexchangeRevisionAssets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataexchangeRevisionAssetsData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                comment: core::default::Default::default(),
                data_set_id: self.data_set_id,
                finalized: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                asset: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}
pub struct DataexchangeRevisionAssetsRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self { shared, base }
    }
}
impl DataexchangeRevisionAssetsRef {
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
    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.comment", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.created_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.data_set_id", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `finalized` after provisioning.\n"]
    pub fn finalized(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.finalized", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> PrimExpr<bool> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.force_destroy", self.extract_ref()),
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
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.updated_at", self.extract_ref()),
        )
    }
    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataexchangeRevisionAssetsTimeoutsElRef {
        DataexchangeRevisionAssetsTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl
{
    kms_key_arn: PrimField<String>,
}
impl
    DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl
{
}
impl ToListMappable for DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl { type O = BlockAssignable < DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl > ; fn do_map (self , base : String) -> Self :: O { BlockAssignable :: Dynamic (DynamicBlock { for_each : format ! ("${{{}}}" , base) , iterator : "each" . into () , content : self , }) } }
pub struct BuildDataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl
{
    #[doc = ""]
    pub kms_key_arn: PrimField<String>,
}
impl BuildDataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl { pub fn build (self) -> DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl { DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl { kms_key_arn : self . kms_key_arn , } } }
pub struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantElRef
{
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantElRef { fn new (shared : StackShared , base : String) -> DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantElRef { DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantElRef { shared : shared , base : base . to_string () , } } }
impl DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantElRef { fn shared (& self) -> & StackShared { & self . shared } # [doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"] pub fn kms_key_arn (& self) -> PrimExpr < String > { PrimExpr :: new (self . shared () . clone () , format ! ("{}.kms_key_arn" , self . base)) } }
#[derive(Serialize, Default)]
struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElDynamic { kms_keys_to_grant : Option < DynamicBlock < DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl >> , }
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl { bucket : PrimField < String > , # [serde (skip_serializing_if = "Option::is_none")] key_prefixes : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] keys : Option < SetField < PrimField < String > > > , # [serde (skip_serializing_if = "Option::is_none")] kms_keys_to_grant : Option < Vec < DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl > > , dynamic : DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElDynamic , }
impl DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl {
    #[doc = "Set the field `key_prefixes`.\n"]
    pub fn set_key_prefixes(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.key_prefixes = Some(v.into());
        self
    }
    #[doc = "Set the field `keys`.\n"]
    pub fn set_keys(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.keys = Some(v.into());
        self
    }
    #[doc = "Set the field `kms_keys_to_grant`.\n"]
    pub fn set_kms_keys_to_grant(
        mut self,
        v : impl Into < BlockAssignable < DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantEl >>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.kms_keys_to_grant = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.kms_keys_to_grant = Some(d);
            }
        }
        self
    }
}
impl ToListMappable
    for DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl
{
    type O = BlockAssignable<
        DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl,
    >;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
}
impl BuildDataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl {
    pub fn build(
        self,
    ) -> DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl {
        DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl {
            bucket: self.bucket,
            key_prefixes: core::default::Default::default(),
            keys: core::default::Default::default(),
            kms_keys_to_grant: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElRef {
        DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `key_prefixes` after provisioning.\n"]
    pub fn key_prefixes(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.key_prefixes", self.base))
    }
    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.keys", self.base))
    }
    #[doc = "Get a reference to the value of field `kms_keys_to_grant` after provisioning.\n"]    pub fn kms_keys_to_grant (& self) -> ListRef < DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElKmsKeysToGrantElRef >{
        ListRef::new(
            self.shared().clone(),
            format!("{}.kms_keys_to_grant", self.base),
        )
    }
}
#[derive(Serialize, Default)]
struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElDynamic {
    asset_source: Option<
        DynamicBlock<
            DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl,
        >,
    >,
}
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_source:
        Option<Vec<DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl>>,
    dynamic: DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElDynamic,
}
impl DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl {
    #[doc = "Set the field `asset_source`.\n"]
    pub fn set_asset_source(
        mut self,
        v: impl Into<
            BlockAssignable<
                DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceEl,
            >,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.asset_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.asset_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl {
    type O = BlockAssignable<DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl {}
impl BuildDataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl {
    pub fn build(self) -> DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl {
        DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl {
            asset_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElRef {
        DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `access_point_alias` after provisioning.\n"]
    pub fn access_point_alias(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_point_alias", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(
            self.shared().clone(),
            format!("{}.access_point_arn", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `asset_source` after provisioning.\n"]
    pub fn asset_source(
        &self,
    ) -> ListRef<DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElAssetSourceElRef>
    {
        ListRef::new(self.shared().clone(), format!("{}.asset_source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl {
    bucket: PrimField<String>,
    key: PrimField<String>,
}
impl DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl {}
impl ToListMappable for DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl {
    type O = BlockAssignable<DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl {
    #[doc = ""]
    pub bucket: PrimField<String>,
    #[doc = ""]
    pub key: PrimField<String>,
}
impl BuildDataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl {
    pub fn build(self) -> DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl {
        DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl {
            bucket: self.bucket,
            key: self.key,
        }
    }
}
pub struct DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceElRef {
        DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }
    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElDynamic {
    asset_source:
        Option<DynamicBlock<DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl>>,
}
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsAssetElImportAssetsFromS3El {
    #[serde(skip_serializing_if = "Option::is_none")]
    asset_source: Option<Vec<DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl>>,
    dynamic: DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElDynamic,
}
impl DataexchangeRevisionAssetsAssetElImportAssetsFromS3El {
    #[doc = "Set the field `asset_source`.\n"]
    pub fn set_asset_source(
        mut self,
        v: impl Into<
            BlockAssignable<DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceEl>,
        >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.asset_source = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.asset_source = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataexchangeRevisionAssetsAssetElImportAssetsFromS3El {
    type O = BlockAssignable<DataexchangeRevisionAssetsAssetElImportAssetsFromS3El>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataexchangeRevisionAssetsAssetElImportAssetsFromS3El {}
impl BuildDataexchangeRevisionAssetsAssetElImportAssetsFromS3El {
    pub fn build(self) -> DataexchangeRevisionAssetsAssetElImportAssetsFromS3El {
        DataexchangeRevisionAssetsAssetElImportAssetsFromS3El {
            asset_source: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElRef {
        DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `asset_source` after provisioning.\n"]
    pub fn asset_source(
        &self,
    ) -> ListRef<DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElAssetSourceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.asset_source", self.base))
    }
}
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl {
    filename: PrimField<String>,
}
impl DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl {}
impl ToListMappable for DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl {
    type O = BlockAssignable<DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl {
    #[doc = ""]
    pub filename: PrimField<String>,
}
impl BuildDataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl {
    pub fn build(self) -> DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl {
        DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl {
            filename: self.filename,
        }
    }
}
pub struct DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlElRef {
        DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataexchangeRevisionAssetsAssetElDynamic {
    create_s3_data_access_from_s3_bucket:
        Option<DynamicBlock<DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl>>,
    import_assets_from_s3:
        Option<DynamicBlock<DataexchangeRevisionAssetsAssetElImportAssetsFromS3El>>,
    import_assets_from_signed_url:
        Option<DynamicBlock<DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl>>,
}
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsAssetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create_s3_data_access_from_s3_bucket:
        Option<Vec<DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_assets_from_s3: Option<Vec<DataexchangeRevisionAssetsAssetElImportAssetsFromS3El>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    import_assets_from_signed_url:
        Option<Vec<DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl>>,
    dynamic: DataexchangeRevisionAssetsAssetElDynamic,
}
impl DataexchangeRevisionAssetsAssetEl {
    #[doc = "Set the field `create_s3_data_access_from_s3_bucket`.\n"]
    pub fn set_create_s3_data_access_from_s3_bucket(
        mut self,
        v: impl Into<BlockAssignable<DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.create_s3_data_access_from_s3_bucket = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.create_s3_data_access_from_s3_bucket = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `import_assets_from_s3`.\n"]
    pub fn set_import_assets_from_s3(
        mut self,
        v: impl Into<BlockAssignable<DataexchangeRevisionAssetsAssetElImportAssetsFromS3El>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.import_assets_from_s3 = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.import_assets_from_s3 = Some(d);
            }
        }
        self
    }
    #[doc = "Set the field `import_assets_from_signed_url`.\n"]
    pub fn set_import_assets_from_signed_url(
        mut self,
        v: impl Into<BlockAssignable<DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.import_assets_from_signed_url = Some(v);
            }
            BlockAssignable::Dynamic(d) => {
                self.dynamic.import_assets_from_signed_url = Some(d);
            }
        }
        self
    }
}
impl ToListMappable for DataexchangeRevisionAssetsAssetEl {
    type O = BlockAssignable<DataexchangeRevisionAssetsAssetEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataexchangeRevisionAssetsAssetEl {}
impl BuildDataexchangeRevisionAssetsAssetEl {
    pub fn build(self) -> DataexchangeRevisionAssetsAssetEl {
        DataexchangeRevisionAssetsAssetEl {
            create_s3_data_access_from_s3_bucket: core::default::Default::default(),
            import_assets_from_s3: core::default::Default::default(),
            import_assets_from_signed_url: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}
pub struct DataexchangeRevisionAssetsAssetElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsAssetElRef {
    fn new(shared: StackShared, base: String) -> DataexchangeRevisionAssetsAssetElRef {
        DataexchangeRevisionAssetsAssetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataexchangeRevisionAssetsAssetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }
    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
    #[doc = "Get a reference to the value of field `create_s3_data_access_from_s3_bucket` after provisioning.\n"]
    pub fn create_s3_data_access_from_s3_bucket(
        &self,
    ) -> ListRef<DataexchangeRevisionAssetsAssetElCreateS3DataAccessFromS3BucketElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.create_s3_data_access_from_s3_bucket", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `import_assets_from_s3` after provisioning.\n"]
    pub fn import_assets_from_s3(
        &self,
    ) -> ListRef<DataexchangeRevisionAssetsAssetElImportAssetsFromS3ElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.import_assets_from_s3", self.base),
        )
    }
    #[doc = "Get a reference to the value of field `import_assets_from_signed_url` after provisioning.\n"]
    pub fn import_assets_from_signed_url(
        &self,
    ) -> ListRef<DataexchangeRevisionAssetsAssetElImportAssetsFromSignedUrlElRef> {
        ListRef::new(
            self.shared().clone(),
            format!("{}.import_assets_from_signed_url", self.base),
        )
    }
}
#[derive(Serialize)]
pub struct DataexchangeRevisionAssetsTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}
impl DataexchangeRevisionAssetsTimeoutsEl {
    #[doc = "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}
impl ToListMappable for DataexchangeRevisionAssetsTimeoutsEl {
    type O = BlockAssignable<DataexchangeRevisionAssetsTimeoutsEl>;
    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}
pub struct BuildDataexchangeRevisionAssetsTimeoutsEl {}
impl BuildDataexchangeRevisionAssetsTimeoutsEl {
    pub fn build(self) -> DataexchangeRevisionAssetsTimeoutsEl {
        DataexchangeRevisionAssetsTimeoutsEl {
            create: core::default::Default::default(),
        }
    }
}
pub struct DataexchangeRevisionAssetsTimeoutsElRef {
    shared: StackShared,
    base: String,
}
impl Ref for DataexchangeRevisionAssetsTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataexchangeRevisionAssetsTimeoutsElRef {
        DataexchangeRevisionAssetsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}
impl DataexchangeRevisionAssetsTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
    #[doc = "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
#[derive(Serialize, Default)]
struct DataexchangeRevisionAssetsDynamic {
    asset: Option<DynamicBlock<DataexchangeRevisionAssetsAssetEl>>,
}
