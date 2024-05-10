pub trait AccountStructure {}
impl AccountStructure for Unset {}
impl AccountStructure for Flat {}
impl AccountStructure for Hierarchichal {}

pub trait BlobKind {}
impl BlobKind for Unset {}
impl BlobKind for Block {}
impl BlobKind for Page {}
impl BlobKind for Append {}

pub struct Unset;

pub struct Flat;
pub struct Hierarchichal;

pub struct Block;
pub struct Page;
pub struct Append;
